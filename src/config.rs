use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Config {
    /// The MQTT broker hostname or IP address
    #[arg(short = 'H', long, env = "MQTT_HOST")]
    pub(crate) mqtt_host: String,

    /// The MQTT broker port
    #[arg(short = 'p', long, env = "MQTT_PORT", default_value_t = 1883)]
    pub(crate) mqtt_port: u16,

    /// The username for MQTT authentication
    #[arg(short = 'u', long, env = "MQTT_USERNAME")]
    pub(crate) username: String,

    /// Path to a text file containing the MQTT password
    #[arg(long, env = "MQTT_PASSWORD_FILE")]
    pub(crate) password_file: PathBuf,
}