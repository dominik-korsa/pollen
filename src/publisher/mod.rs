use crate::state::State;

pub mod mqtt;

pub trait Publisher {
    fn publish(&self, state: &State) -> Result<(), rumqttc::ClientError>;
}
