pub const STABLE_COIN: &str = "USDT";
const LOCAL_PATH: &str = "./binance_data/";
const DOWNLOADS_PATH: &str = "downloads/";
const RESULTS_PATH: &str = "results/";


pub struct AssetFile {
    asset: String,
    granularity: String,
    year: i32,
    month: u32,
    month_prefix: String,
}

impl AssetFile {
    pub fn new(asset: &str, granularity: &str, year: i32, month: u32) -> AssetFile {
        let month_prefix = if month < 10 {
            "0".to_string()
        } else {
            String::new()
        };

        AssetFile { asset: asset.to_string(), granularity: granularity.to_string(), year, month, month_prefix }
    }

    pub fn get_display_name(&self) -> String {
        format!("[{}{} {} -> {}/{}]", self.asset, STABLE_COIN, self.granularity, self.month, self.year)
    }
    pub fn get_file_name(&self) -> String {
        format!("{}{}-{}-{}-{}{}", self.asset, STABLE_COIN, self.granularity, self.year, self.month_prefix, self.month)
    }
    pub fn get_download_directory(&self) -> String {
        self.get_local_directory(DOWNLOADS_PATH)
    }
    pub fn get_extract_directory(&self) -> String {
        self.get_local_directory(RESULTS_PATH)
    }

    pub fn get_full_file_name(&self, extension: &str) -> String {
        self.get_file_name() + extension
    }
    pub fn get_download_url(&self, extension: &str) -> String {
        format!("https://data.binance.vision/data/spot/monthly/klines/{}{}/{}/{}", self.asset, STABLE_COIN, self.granularity, self.get_full_file_name(extension))
    }
    pub fn get_cache_directory() -> String {
        format!("{}{}", LOCAL_PATH, DOWNLOADS_PATH)
    }

    fn get_local_directory(&self, directory: &str) -> String {
        format!("{}{}{}{}/{}/", LOCAL_PATH, directory, self.asset, STABLE_COIN, self.granularity)
    }
}

