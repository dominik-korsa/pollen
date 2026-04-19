use std::time::Duration;
use rand::distr::Alphanumeric;
use rand::{Rng, RngExt};
use rumqttc::{Client, Connection, Event, MqttOptions, Packet, QoS};
use crate::publisher::{Publisher, State};

pub struct MqttPublisher {
    mqtt_options: MqttOptions,
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

    fn consume(sent_message_count: i32, client: Client, mut connection: Connection) -> Result<(), rumqttc::ClientError> {
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
                Err(e) => {
                    println!("Connection closed: {:?}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Publisher for MqttPublisher {
    fn publish(&self, state: &State) -> Result<(), rumqttc::ClientError> {
        let mut sent_message_count = 0;

        let (client, connection) = Client::new(self.mqtt_options.clone(), 10);
        client.publish(
            "pollen/cm_uj/state",
            QoS::AtLeastOnce,
            true,
            serde_json::to_vec(state).expect("Failed to serialize state")
        )?;
        sent_message_count += 1;

        Self::consume(sent_message_count, client, connection)
    }
}