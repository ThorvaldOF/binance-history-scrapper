use crate::input::GRANULARITIES;
use crate::utils::month_year::MonthYear;

pub const STABLE_COIN: &str = "USDT";
const LOCAL_PATH: &str = "./binance_data/";
const DOWNLOADS_PATH: &str = "downloads/";
const OUTPUT_PATH: &str = "output/";


pub struct AssetFile {
    asset: String,
    granularity: String,
    month_year: MonthYear,
    ts_factor: u64,
}

impl AssetFile {
    pub fn new(asset: &str, granularity: &str, month_year: MonthYear) -> AssetFile {
        let mut ts_factor: u64 = 0;
        for grn_pair in GRANULARITIES {
            if granularity == grn_pair.0 {
                ts_factor = grn_pair.1;
            }
        }
        if ts_factor == 0 {
            //TODO: better error handling
            panic!("Couldn't define a timestamp factor for your granularity");
        }

        AssetFile { asset: asset.to_string(), granularity: granularity.to_string(), month_year, ts_factor }
    }

    pub fn get_file_name(&self) -> String {
        format!("{}{}-{}-{}-{}", self.asset, STABLE_COIN, self.granularity, self.month_year.get_year(), self.month_year.get_month_string())
    }
    pub fn get_download_directory(&self) -> String {
        self.get_local_directory(DOWNLOADS_PATH)
    }
    pub fn get_extract_directory(&self) -> String {
        format!("{}{}{}/", LOCAL_PATH, OUTPUT_PATH, self.granularity)
    }
    pub fn get_result_file_path(&self) -> String {
        Self::get_result_file_path_from_values(&self.granularity, &self.asset)
    }
    pub fn get_result_file_path_from_values(granularity: &str, asset: &str) -> String {
        format!("{}{}{}/{}{}.bin", LOCAL_PATH, OUTPUT_PATH, granularity, asset, STABLE_COIN)
    }

    pub fn get_full_file_name(&self, extension: &str) -> String {
        self.get_file_name() + extension
    }
    pub fn get_download_url(&self, extension: &str) -> String {
        format!("https://data.binance.vision/data/spot/monthly/klines/{}{}/{}/{}", self.asset, STABLE_COIN, self.granularity, self.get_full_file_name(extension))
    }

    fn get_local_directory(&self, directory: &str) -> String {
        format!("{}{}{}/{}{}/", LOCAL_PATH, directory, self.granularity, self.asset, STABLE_COIN)
    }
    pub fn get_ts_factor(&self) -> u64 {
        self.ts_factor
    }
}

