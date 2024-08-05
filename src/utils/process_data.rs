use chrono::{Datelike, Local};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use crate::{BINANCE_BIRTH};

#[derive(Clone)]
pub struct ProcessData {
    pub granularity: String,
    pub asset: String,
    pub clear_cache: bool,
    end: (i32, u32),
    multi_progress: MultiProgress,
    progress_bar: Option<ProgressBar>,
}

impl ProcessData {
    pub fn new(granularity: &str, asset: &str, clear_cache: bool, multi_progress: MultiProgress) -> ProcessData {
        let end = get_end_date();
        ProcessData { granularity: granularity.to_string(), asset: asset.to_string(), clear_cache, multi_progress, end, progress_bar: None }
    }

    pub fn init_progress_bar(&mut self) {
        if self.progress_bar.is_some() {
            return;
        }
        let full_years = self.end.0 - BINANCE_BIRTH - 1;
        let bar_size = full_years as u32 * 12 + self.end.1;

        let pb = self.multi_progress.add(ProgressBar::new(bar_size as u64));
        pb.set_style(Self::get_progress_bar_style("white/grey"));
        pb.set_prefix(self.asset.clone());
        self.progress_bar = Some(pb);
    }

    pub fn finish_progress_bar(&mut self, message: &str, style: &str) {
        if let Some(pb) = self.progress_bar.as_mut() {
            pb.set_style(Self::get_progress_bar_style(style));
            pb.finish_with_message(message.to_string());
        }
    }
    pub fn increment_progress_bar(&mut self) {
        if let Some(pb) = self.progress_bar.as_mut() {
            pb.inc(1);
        }
    }

    pub fn get_end(&self) -> (i32, u32) {
        self.end
    }
    pub fn get_clear_cache(&self) -> bool {
        self.clear_cache
    }
    pub fn get_asset(&self) -> String {
        self.asset.clone()
    }
    pub fn get_granularity(&self) -> String {
        self.granularity.clone()
    }
    fn get_progress_bar_style(color: &str) -> ProgressStyle {
        let template = format!("{}{}{}", "[{prefix}] {bar:40.", color, "} {pos:>7}/{len:10} {msg}");
        ProgressStyle::with_template(
            &template,
        )
            .unwrap()
            .progress_chars("##-")
    }
}

fn get_end_date() -> (i32, u32) {
    let today = Local::now();
    let start_date: (i32, u32) = if today.month() <= 2 {
        let new_month = if today.month() == 2 {
            12
        } else {
            11
        };
        (today.year() - 1, new_month)
    } else {
        (today.year(), today.month() - 2)
    };
    start_date
}