use chrono::{Datelike, Local};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use crate::{BINANCE_BIRTH};
use crate::utils::month_year::MonthYear;

#[derive(Clone)]
pub struct ProcessData {
    granularity: String,
    asset: String,
    start: Option<MonthYear>,
    end: MonthYear,
    progress_bar: Option<ProgressBar>,
}

impl ProcessData {
    pub fn new(granularity: &str, asset: &str, start: Option<MonthYear>) -> ProcessData {
        let end = get_end_date();
        ProcessData { granularity: granularity.to_string(), start, asset: asset.to_string(), end, progress_bar: None }
    }

    pub fn init_progress_bar(&mut self, multi_progress: &MultiProgress) {
        if self.progress_bar.is_some() {
            return;
        }
        let full_years = self.end.get_year() - BINANCE_BIRTH;
        let bar_size = full_years * 12;

        let pb = multi_progress.add(ProgressBar::new(bar_size as u64));
        pb.set_style(Self::get_progress_bar_style("white/grey"));
        pb.set_prefix(format!("[{}]", self.asset.clone()));
        self.progress_bar = Some(pb);
    }


    pub fn finish_progress_bar(&mut self, multi_progress: &MultiProgress) {
        if let Some(pb) = self.progress_bar.as_mut() {
            multi_progress.remove(pb);
        }
    }
    pub fn increment_progress_bar(&mut self) {
        if let Some(pb) = self.progress_bar.as_mut() {
            pb.inc(1);
        }
    }
    pub fn get_start(&self) -> Option<MonthYear> {
        self.start.clone()
    }
    pub fn set_start(&mut self, start: MonthYear) {
        self.start = Some(start);
    }

    pub fn get_end(&self) -> MonthYear {
        self.end.clone()
    }
    pub fn get_asset(&self) -> String {
        self.asset.clone()
    }
    pub fn get_granularity(&self) -> String {
        self.granularity.clone()
    }
    pub fn get_progress_bar_style(color: &str) -> ProgressStyle {
        let template = format!("{}{}{}", "{prefix:<9} {bar:50.", color, "} {percent:>4}% {msg:>4}");
        ProgressStyle::with_template(
            &template,
        )
            .unwrap()
            .progress_chars("#>-")
    }
}

fn get_end_date() -> MonthYear {
    let today = Local::now();
    let end_date: MonthYear = if today.month() <= 2 {
        let new_month = if today.month() == 2 {
            12
        } else {
            11
        };
        MonthYear::new(new_month, today.year() - 1)
    } else {
        MonthYear::new((today.month() - 2) as u8, today.year())
    };
    end_date
}