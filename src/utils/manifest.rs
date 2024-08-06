use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct TimePeriod {
    start: u64,
    end: u64,
}

impl TimePeriod {
    pub fn new(start: u64, end: u64) -> TimePeriod {
        TimePeriod { start, end }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatePeriod {
    start: String,
    end: String,
}

impl DatePeriod {
    pub fn new(start: &str, end: &str) -> DatePeriod {
        DatePeriod { start: start.to_string(), end: end.to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    down_times: Vec<TimePeriod>,
    assets: HashMap<String, DatePeriod>,
    #[serde(skip_serializing)]
    granularity: String,
}


impl Manifest {
    pub fn new(granularity: &str) -> Manifest {
        Manifest { down_times: vec![], assets: HashMap::new(), granularity: granularity.to_string() }
    }
    pub fn add_down_time(&mut self, time_period: TimePeriod) {
        for down in &self.down_times {
            if down.start == time_period.start && down.end == time_period.end {
                return;
            }
        }
        self.down_times.push(time_period);
    }
    pub fn add_asset(&mut self, asset: &str, date_period: DatePeriod) {
        self.assets.insert(asset.to_string(), date_period);
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(format!("./binance_data/results/{}/manifest.json", self.granularity))?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}