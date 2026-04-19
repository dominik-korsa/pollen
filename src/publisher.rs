use crate::data_source::PollenReportMetadata;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct State {
    #[serde(flatten)]
    pub metadata: PollenReportMetadata
}

pub trait Publisher {
    fn publish(&self, state: &State) -> Result<(), rumqttc::ClientError>;
}
