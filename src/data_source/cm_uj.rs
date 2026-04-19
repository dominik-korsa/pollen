use std::io;
use htmd::HtmlToMarkdown;
use scraper::{ElementRef, Html, Selector};
use regex::Regex;
use crate::data_source::{DataSource, Pollen, PollenReport};

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
}

impl CmUjDataSource {
    pub fn new(fetcher: Box<dyn HtmlFetcher>) -> Self {
        let content_selector = Selector::parse(".tekst_glowny").unwrap();
        let date_selector = Selector::parse("strong").unwrap();
        let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

        let markdown_converter = HtmlToMarkdown::builder()
            .skip_tags(vec!["table"])
            .build();

        Self {
            fetcher,
            content_selector,
            date_selector,
            date_regex,
            markdown_converter,
        }
    }

    fn extract_date(&self, content: ElementRef) -> Option<String> {
        content.select(&self.date_selector)
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|date| self.date_regex.is_match(date))
            .next()
    }

    fn extract_description(&self, content: ElementRef) -> io::Result<String> {
        self.markdown_converter.convert(&content.inner_html())
    }

    fn extract_pollen_list(&self) -> Vec<Pollen> {
        vec![] // TODO: Implement
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
        let pollen_list = self.extract_pollen_list();

        Ok(PollenReport {
            date,
            description,
            pollen_list,
        })
    }
}