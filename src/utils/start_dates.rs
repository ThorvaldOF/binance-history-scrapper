use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::utils::month_year::MonthYear;

const START_DATES_PATH: &str = "./start_dates.json";

#[derive(Serialize, Deserialize)]
pub struct StartDates {
    start_dates: HashMap<String, MonthYear>,
}

impl StartDates {
    pub fn load() -> StartDates {
        let start_dates = match fs::read_to_string(START_DATES_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(_) => HashMap::new(),
        };
        StartDates { start_dates }
    }
    pub fn save(&self) {
        let content = serde_json::to_string_pretty(&self.start_dates).unwrap();
        let mut file = File::create(START_DATES_PATH).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn get_start_date(&self, asset: &str) -> Option<&MonthYear> {
        self.start_dates.get(asset)
    }
    pub fn set_start_date(&mut self, asset: &str, month_year: MonthYear) {
        self.start_dates.insert(asset.to_string(), month_year);
    }
}
