use std::io;
use htmd::HtmlToMarkdown;
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use regex::Regex;
use crate::data_source::{DataSource, Pollen, PollenLevel, PollenReport, Trend};

pub trait HtmlFetcher {
    fn fetch(&self) -> Result<String, Box<dyn std::error::Error>>;
}

pub struct HttpHtmlFetcher {
    url: String,
}

impl HttpHtmlFetcher {
    pub(crate) fn new(url: String) -> Self {
        Self { url }
    }
}

impl HtmlFetcher for HttpHtmlFetcher {
    fn fetch(&self) -> Result<String, Box<dyn std::error::Error>> {
        let response = reqwest::blocking::get(&self.url)?.error_for_status()?;
        let html_data = response.text()?;
        Ok(html_data)
    }
}

pub struct CmUjDataSource {
    fetcher: Box<dyn HtmlFetcher>,
    content_selector: Selector,
    date_selector: Selector,
    date_regex: Regex,
    markdown_converter: HtmlToMarkdown,
    table_selector: Selector,
    table_row_selector: Selector,
}

impl CmUjDataSource {
    pub fn new(fetcher: Box<dyn HtmlFetcher>) -> Self {
        let content_selector = Selector::parse(".tekst_glowny").unwrap();
        let date_selector = Selector::parse("strong").unwrap();
        let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

        let markdown_converter = HtmlToMarkdown::builder()
            .skip_tags(vec!["table"])
            .build();

        let table_selector = Selector::parse("div.table-responsive > table.table").unwrap();
        let table_row_selector = Selector::parse("tr:not(:first-child)").unwrap();

        Self {
            fetcher,
            content_selector,
            date_selector,
            date_regex,
            markdown_converter,
            table_selector,
            table_row_selector,
        }
    }

    fn text_of(element: ElementRef) -> String {
        element.text().collect::<String>().trim().to_string()
    }

    fn extract_date(&self, content: ElementRef) -> Option<String> {
        content.select(&self.date_selector)
            .map(|el| Self::text_of(el))
            .filter(|date| self.date_regex.is_match(date))
            .next()
    }

    fn extract_description(&self, content: ElementRef) -> io::Result<String> {
        self.markdown_converter.convert(&content.inner_html())
    }

    fn map_row(&self, row: ElementRef) -> io::Result<Pollen> {
        let (name, level, trend) = row.children()
            .filter_map(ElementRef::wrap)
            .map(|el| Self::text_of(el))
            .collect_tuple()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid number of row children"))?;

        let level: PollenLevel = match level.as_str() {
            "stężenie niskie" => PollenLevel::Low,
            "stężenie średnie" => PollenLevel::Medium,
            "stężenie wysokie" => PollenLevel::High,
            _ => return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid pollen level {level}")).into()),
        };
        let trend: Trend = match trend.as_str() {
            "spadek" => Trend::Down,
            "wzrost" => Trend::Up,
            "bez zmian" => Trend::Same,
            _ => return Err(io::Error::new(io::ErrorKind::Other, format!("Invalid pollen trend {trend}")).into()),
        };

        Ok(Pollen {
            name,
            level,
            trend,
        })
    }

    fn extract_pollen_list(&self, content: ElementRef) -> io::Result<Vec<Pollen>> {
        let Some(table) = content.select(&self.table_selector).next() else {
            return Err(io::Error::new(io::ErrorKind::Other, "Table not found").into());
        };

        let pollen_vec: io::Result<Vec<Pollen>> = table
            .select(&self.table_row_selector)
            .map(|row| self.map_row(row))
            .collect();
        pollen_vec
    }
}

impl DataSource for CmUjDataSource {
    fn get_report(&self) -> Result<PollenReport, Box<dyn std::error::Error>> {

        let html = self.fetcher.fetch()?;
        let document = Html::parse_document(&html);
        let Some(content) = document.select(&self.content_selector).next() else {
            return Err(io::Error::new(io::ErrorKind::Other, ".tekst_glowny tag not found").into());
        };

        let date = self.extract_date(content);
        let description = self.extract_description(content)?;
        let pollen_list = self.extract_pollen_list(content)?;

        Ok(PollenReport {
            date,
            description,
            pollen_list,
        })
    }
}