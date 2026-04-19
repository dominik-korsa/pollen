use std::fs;
use clap::Parser;
use crate::config::Config;

use publisher::mqtt::MqttPublisher;
use crate::data_source::cm_uj::HttpHtmlFetcher;
use crate::data_source::DataSource;
use crate::publisher::Publisher;

mod data_source;
mod publisher;
mod config;
mod state;
mod pollen_storage;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    let fetcher = Box::new(HttpHtmlFetcher::new(
        "https://toksy-alergo.cm-uj.krakow.pl/pl/komunikat-pylkowy-dla-alergikow-malopolska/".to_string()
    ));
    let data_source = data_source::cm_uj::CmUjDataSource::new(
        fetcher
    );
    let pollen_storage = pollen_storage::null::NullPollenStorage;
    let state_serializer = state::StateSerializer::new(pollen_storage);

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

    let state = state_serializer.create_state(report)?;

    publisher.publish(&state)?;

    Ok(())
}
