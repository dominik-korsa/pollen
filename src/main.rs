use crate::data_source::cm_uj::HttpHtmlFetcher;
use crate::data_source::DataSource;
use crate::mqtt_publisher::MqttPublisher;
use crate::publisher::{Publisher, State};

mod data_source;
pub mod publisher;
mod mqtt_publisher;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fetcher = Box::new(HttpHtmlFetcher::new(
        "https://toksy-alergo.cm-uj.krakow.pl/pl/komunikat-pylkowy-dla-alergikow-malopolska/".to_string()
    ));
    let data_source = data_source::cm_uj::CmUjDataSource::new(
        fetcher
    );

    let publisher = MqttPublisher::new(
        "test.mosquitto.org".to_string(),
        1883,
        "test".to_string(),
        "test".to_string()
    );

    let report = data_source.get_report()?;
    println!("{:?}", report);

    let state = State {
        metadata: report.metadata,
    };
    publisher.publish(&state)?;

    Ok(())
}
