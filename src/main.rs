use std::fs;
use clap::Parser;
use crate::config::Config;
use crate::data_source::cm_uj::HttpHtmlFetcher;
use crate::data_source::DataSource;
use crate::mqtt_publisher::MqttPublisher;
use crate::publisher::{Publisher, State};

mod data_source;
pub mod publisher;
mod mqtt_publisher;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let fetcher = Box::new(HttpHtmlFetcher::new(
        "https://toksy-alergo.cm-uj.krakow.pl/pl/komunikat-pylkowy-dla-alergikow-malopolska/".to_string()
    ));
    let data_source = data_source::cm_uj::CmUjDataSource::new(
        fetcher
    );

    let publisher = MqttPublisher::new(
        config.mqtt_host,
        config.mqtt_port,
        config.username,
        fs::read_to_string(config.password_file)?
            .trim()
            .to_string(),
    );

    let report = data_source.get_report()?;
    println!("{:?}", report);

    let state = State {
        metadata: report.metadata,
    };
    publisher.publish(&state)?;

    Ok(())
}
