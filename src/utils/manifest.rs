use serde::{Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct TimePeriod {
    start: u64,
    end: u64,
}

impl TimePeriod {
    pub fn new(start: u64, end: u64) -> TimePeriod {
        TimePeriod { start, end }
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
        let mut ref_down_times = self.down_times.clone();
        if ref_down_times.is_empty() {
            return;
        }
        ref_down_times.sort_by_key(|x| x.start);

        let mut merged_down_times: Vec<TimePeriod> = vec![];
        let mut current_down_time = ref_down_times[0].clone();

        for period in ref_down_times.iter().skip(1) {
            if period.start <= current_down_time.end {
                current_down_time.end = current_down_time.end.max(period.end);
            } else {
                merged_down_times.push(current_down_time);
                current_down_time = period.clone();
            }
        }

        merged_down_times.push(current_down_time);

        self.down_times = merged_down_times;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_overlap() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![
                TimePeriod { start: 1, end: 3 },
                TimePeriod { start: 5, end: 7 },
                TimePeriod { start: 9, end: 11 },
            ],
        };

        m.concat_down_times();

        let expected = vec![
            TimePeriod { start: 1, end: 3 },
            TimePeriod { start: 5, end: 7 },
            TimePeriod { start: 9, end: 11 },
        ];

        assert_eq!(m.down_times, expected);
    }

    #[test]
    fn test_overlap() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![
                TimePeriod { start: 1, end: 5 },
                TimePeriod { start: 2, end: 6 },
                TimePeriod { start: 8, end: 10 },
                TimePeriod { start: 9, end: 11 },
            ],
        };

        m.concat_down_times();

        let expected = vec![
            TimePeriod { start: 1, end: 6 },
            TimePeriod { start: 8, end: 11 },
        ];

        assert_eq!(m.down_times, expected);
    }

    #[test]
    fn test_contiguous_periods() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![
                TimePeriod { start: 1, end: 3 },
                TimePeriod { start: 3, end: 5 },
                TimePeriod { start: 5, end: 7 },
            ],
        };

        m.concat_down_times();

        let expected = vec![
            TimePeriod { start: 1, end: 7 },
        ];

        assert_eq!(m.down_times, expected);
    }

    #[test]
    fn test_contained_periods() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![
                TimePeriod { start: 1, end: 10 },
                TimePeriod { start: 2, end: 5 },
                TimePeriod { start: 3, end: 4 },
                TimePeriod { start: 11, end: 15 },
            ],
        };

        m.concat_down_times();

        let expected = vec![
            TimePeriod { start: 1, end: 10 },
            TimePeriod { start: 11, end: 15 },
        ];

        assert_eq!(m.down_times, expected);
    }

    #[test]
    fn test_single_period() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![
                TimePeriod { start: 1, end: 3 },
            ],
        };

        m.concat_down_times();

        let expected = vec![
            TimePeriod { start: 1, end: 3 },
        ];

        assert_eq!(m.down_times, expected);
    }

    #[test]
    fn test_empty_periods() {
        let mut m = Manifest {
            assets: HashMap::new(),
            granularity: "1m".to_string(),
            down_times: vec![],
        };

        m.concat_down_times();

        let expected: Vec<TimePeriod> = vec![];

        assert_eq!(m.down_times, expected);
    }
}
