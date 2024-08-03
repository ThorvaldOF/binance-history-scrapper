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
struct Period {
    down_times: Vec<TimePeriod>,
    assets: HashMap<String, DatePeriod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    periods: HashMap<String, Period>,
}

impl Manifest {
    pub fn new() -> Manifest {
        Manifest { periods: Default::default() }
    }
    pub fn add_down_time(&mut self, period: &str, start: u64, end: u64) {
        self.check_period(period);
        let mut current = self.periods.get_mut(period).unwrap();
        for down in &current.down_times {
            if down.start == start && down.end == end {
                return;
            }
        }
        current.down_times.push(TimePeriod { start, end });
    }
    pub fn add_asset(&mut self, period: &str, asset: &str, date_period: DatePeriod) {
        self.check_period(period);
        let mut current = self.periods.get_mut(period).unwrap();
        current.assets.insert(asset.to_string(), date_period);
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create("./binance_data/manifest.json")?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    fn check_period(&mut self, period: &str) {
        if self.periods.contains_key(period) {
            return;
        }
        self.periods.insert(period.to_string(), Period { down_times: vec![], assets: HashMap::new() });
    }
}