use serde::{Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Debug, Clone)]
pub struct TimePeriod {
    start: u64,
    end: u64,
}

impl TimePeriod {
    pub fn new(start: u64, end: u64) -> TimePeriod {
        TimePeriod { start, end }
    }

    fn is_included(&self, other: &TimePeriod) -> bool {
        if self.start >= other.start && self.end <= other.end {
            return true;
        }
        false
    }
}


#[derive(Serialize, Debug)]
pub struct Manifest {
    down_times: Vec<TimePeriod>,
    assets: HashMap<String, TimePeriod>,
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
    pub fn add_asset(&mut self, asset: &str, time_period: TimePeriod) {
        self.assets.insert(asset.to_string(), time_period);
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        self.concat_down_times();
        let json = serde_json::to_string_pretty(&self)?;
        let dir_path = format!("./binance_data/results/{}", self.granularity);
        fs::create_dir_all(&dir_path)?;
        let mut file = File::create(format!("{}/manifest.json", dir_path))?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    fn concat_down_times(&mut self) {
        let mut is_changed = true;
        let mut new_down_times: Vec<TimePeriod> = vec![];
        while is_changed {
            is_changed = false;
            let res = self.iter_concat_down_times();
            if new_down_times.len() != res.len() {
                is_changed = true;
                new_down_times = res;
            }
        }
        self.down_times = new_down_times;
    }
    fn iter_concat_down_times(&self) -> Vec<TimePeriod> {
        let mut last: TimePeriod = TimePeriod::new(0, 0);
        let mut down_times: Vec<TimePeriod> = vec![];
        for current in &self.down_times {
            if current.is_included(&last) {
                continue;
            }
            down_times.push(current.clone());
            last = current.clone();
        }
        down_times
    }
}