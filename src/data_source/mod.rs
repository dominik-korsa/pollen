use serde::Serialize;

pub mod cm_uj;

#[derive(Debug)]
pub enum PollenLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
pub enum Trend {
    Up,
    Down,
    Same,
}

#[derive(Debug)]
pub struct Pollen {
    pub name: String,
    pub level: PollenLevel,
    pub trend: Trend,
}

#[derive(Debug, Serialize)]
pub struct PollenReportMetadata {
    /// Report date in the ISO 8601 format (YYYY-MM-DD).
    date: Option<String>,
    /// Text description of the report as Markdown.
    description: String,
}

#[derive(Debug)]
pub struct PollenReport {
    pub metadata: PollenReportMetadata,
    pub pollen_list: Vec<Pollen>,
}

pub trait DataSource {
    fn get_report(&self) -> Result<PollenReport, Box<dyn std::error::Error>>;
}