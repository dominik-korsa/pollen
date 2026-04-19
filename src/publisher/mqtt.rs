use std::time::Duration;
use rand::distr::Alphanumeric;
use rand::{Rng, RngExt};
use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use rumqttc::ConnectionError::MqttState;
use rumqttc::StateError::ConnectionAborted;
use serde::Serialize;
use serde_json::json;
use crate::publisher::Publisher;
use crate::state::State;

pub struct MqttPublisher {
    mqtt_options: MqttOptions,
}

#[derive(Debug, Serialize)]
struct DeviceConfig {
    identifiers: Vec<&'static str>,
    name: &'static str,
    manufacturer: &'static str,
}

#[derive(Debug, Serialize)]
struct SensorConfig<'a> {
    name: String,
    unique_id: String,
    object_id: String,
    state_topic: &'a str,
    value_template: String,
    icon: &'a str,
    device: &'a DeviceConfig,
    #[serde(flatten)]
    extra_attributes: serde_json::Value,
}

impl MqttPublisher {
    fn generate_client_id(rng: impl Rng) -> String {
        let suffix: String = rng
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();
        format!("pollen-{}", suffix)
    }

    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        let client_id = Self::generate_client_id(rand::rng());
        let mut mqtt_options = MqttOptions::new(&client_id, host, port);
        mqtt_options.set_credentials(username, password);
        mqtt_options.set_keep_alive(Duration::from_secs(5));
        Self { mqtt_options }
    }

    fn consume(sent_message_count: i32, client: Client, mut connection: Connection) -> Result<(), Box<dyn std::error::Error>> {
        let mut remaining_acks = sent_message_count;

        for notification in connection.iter() {
            match notification {
                Ok(Event::Incoming(Packet::PubAck(_))) => {
                    remaining_acks -= 1;
                    if remaining_acks <= 0 {
                        println!("All messages confirmed by broker. Disconnecting...");
                        client.disconnect()?;
                    }
                }
                Err(MqttState(ConnectionAborted)) => {
                    break;
                }
                Err(e) => {
                    return Err(e.into());
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Publisher for MqttPublisher {
    fn publish(&self, state: &State) -> Result<(), Box<dyn std::error::Error>> {
        let mut sent_message_count = 0;
        let state_topic = "pollen/cm_uj/state";

        let (client, connection) = Client::new(self.mqtt_options.clone(), 256);
        client.publish(
            state_topic,
            QoS::AtLeastOnce,
            true,
            serde_json::to_vec(state).expect("Failed to serialize state")
        )?;
        sent_message_count += 1;

        let device_config = DeviceConfig {
            identifiers: vec!["pollen_cm_uj"],
            name: "Komunikat pyłkowy CM UJ",
            manufacturer: "CM UJ",
        };

        let date_config = SensorConfig {
            name: "Data raportu".to_string(),
            unique_id: "pollen_cm_uj_date".to_string(),
            object_id: "pollen_cm_uj_date".to_string(),
            state_topic,
            value_template: "{{ value_json.date }}".to_string(),
            icon: "mdi:calendar-clock",
            device: &device_config,
            extra_attributes: json!({
                "device_class": "date",
            }),
        };
        client.publish(
            "homeassistant/sensor/pollen_cm_uj/date/config",
            QoS::AtLeastOnce,
            true,
            serde_json::to_vec(&date_config).expect("Failed to serialize")
        )?;
        sent_message_count += 1;

        let description_config = SensorConfig {
            name: "Opis raportu".to_string(),
            unique_id: "pollen_cm_uj_description".to_string(),
            object_id: "pollen_cm_uj_description".to_string(),
            state_topic,
            value_template: "{{ 'Dostępny' if value_json.description else 'Brak' }}".to_string(),
            icon: "mdi:text-box-outline",
            device: &device_config,
            extra_attributes: json!({
                "json_attributes_topic": "pollen/cm_uj/state",
                "json_attributes_template": "{{ {'markdown': value_json.description} | tojson }}",
            }),
        };
        client.publish(
            "homeassistant/sensor/pollen_cm_uj/description/config",
            QoS::AtLeastOnce,
            true,
            serde_json::to_vec(&description_config).expect("Failed to serialize")
        )?;
        sent_message_count += 1;

        for (id, pollen) in state.pollen.iter() {
            let sensor_id = format!("pollen_cm_uj_pollen_{}", id);
            let pollen_config = SensorConfig {
                name: format!("Stężenie {}", pollen.pollen_name),
                unique_id: sensor_id.clone(),
                object_id: sensor_id,
                state_topic,
                value_template: format!("{{{{ value_json.pollen.{id}.level_numeric }}}}"),
                icon: "mdi:flower-pollen",
                device: &device_config,
                extra_attributes: json!({
                    "state_class": "measurement",
                    "json_attributes_topic": "pollen/cm_uj/state",
                    "json_attributes_template": format!("{{{{ value_json.pollen.{id} | tojson }}}}"),
                }),
            };
            client.publish(
                format!("homeassistant/sensor/pollen_cm_uj/pollen-{id}/config"),
                QoS::AtLeastOnce,
                true,
                serde_json::to_vec(&pollen_config).expect("Failed to serialize")
            )?;
            sent_message_count += 1;
        }

        Self::consume(sent_message_count, client, connection)
    }
}