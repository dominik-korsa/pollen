use crate::data_source::cm_uj::HttpHtmlFetcher;
use crate::data_source::DataSource;
mod data_source;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetcher = Box::new(HttpHtmlFetcher::new(
        "https://toksy-alergo.cm-uj.krakow.pl/pl/komunikat-pylkowy-dla-alergikow-malopolska/".to_string()
    ));
    let data_source = data_source::cm_uj::CmUjDataSource::new(
        fetcher
    );
    let report = data_source.get_report()?;
    println!("{:?}", report);
    Ok(())
}
