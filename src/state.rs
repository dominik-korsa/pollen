use std::collections::HashMap;
use serde::Serialize;
use crate::data_source::{Pollen, PollenLevel, PollenReport, PollenReportMetadata, Trend};
use crate::pollen_storage::{generate_id_for_name, PollenStorage};

#[derive(Debug, Serialize)]
pub struct StatePollen {
    pub name: String,
    pub level_numeric: i8,
    pub level_text: &'static str,
    pub trend_value: &'static str,
    pub trend_text: &'static str,
}

impl From<&Pollen> for StatePollen {
    fn from(value: &Pollen) -> Self {
        let (level_numeric, level_text) = match value.level {
            PollenLevel::High => (3, "wysokie"),
            PollenLevel::Medium => (2, "średnie"),
            PollenLevel::Low => (1, "niskie"),
        };

        let (trend_value, trend_text) = match value.trend {
            Trend::Up => ("up", "wzrost"),
            Trend::Same => ("same", "bez zmian"),
            Trend::Down => ("down", "spadek"),
        };

        StatePollen {
            name: value.name.clone(),
            level_numeric,
            level_text,
            trend_value,
            trend_text,
        }
    }
}

impl StatePollen {
    fn none(name: String) -> Self {
        StatePollen {
            name,
            level_numeric: 0,
            level_text: "brak",
            trend_value: "same",
            trend_text: "bez zmian",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct State {
    #[serde(flatten)]
    pub metadata: PollenReportMetadata,

    pub pollen: HashMap<String, StatePollen>,
}

pub struct StateSerializer<S: PollenStorage> {
    storage: S,
}

impl<S: PollenStorage> StateSerializer<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
        }
    }

    /// Prepare the report for publishing by including
    /// entity IDs for all previously encountered pollen types.
    pub fn create_state(&self, report: PollenReport) -> Result<State, Box<dyn std::error::Error>> {
        let mut map = self.storage.get_map()?;
        for pollen in &report.pollen_list {
            if map.contains_right(pollen.name.as_str()) {
                continue
            }
            map.insert(generate_id_for_name(&pollen.name), pollen.name.clone());
        }

        self.storage.set_map(&map)?;

        let pollen: HashMap<String, StatePollen> = map.into_iter()
            .map(|(id, name)| {
                let state_pollen = report.pollen_list.iter()
                    .find(|pollen| pollen.name == name)
                    .map(StatePollen::from)
                    .unwrap_or(StatePollen::none(name));
                (id, state_pollen)
            })
            .collect();

        Ok(State {
            metadata: report.metadata,
            pollen,
        })
    }
}