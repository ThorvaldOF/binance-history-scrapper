#[cfg(test)]
pub mod test_asset_file {
    use ureq::{Agent, AgentBuilder};
    use crate::utils::asset_file::AssetFile;

    #[test]
    fn test_get_display_name() {
        let asset_file = AssetFile::new("BTC", "1h", 2024, 3, AgentBuilder::new().build());
        assert_eq!(asset_file.get_display_name(), "[BTCUSDT 1h -> 3/2024]");
    }

    #[test]
    fn test_get_file_name() {
        let asset_file = AssetFile::new("ETH", "1d", 2023, 12, AgentBuilder::new().build());
        assert_eq!(asset_file.get_file_name(), "ETHUSDT-1d-2023-12");
    }

    #[test]
    fn test_get_download_directory() {
        let asset_file = AssetFile::new("BNB", "1m", 2022, 5, AgentBuilder::new().build());
        assert_eq!(asset_file.get_download_directory(), "./binance_data/downloads/BNBUSDT/1m/");
    }

    #[test]
    fn test_get_extract_directory() {
        let asset_file = AssetFile::new("LTC", "1w", 2021, 10, AgentBuilder::new().build());
        assert_eq!(asset_file.get_extract_directory(), "./binance_data/results/LTCUSDT/1w/");
    }

    #[test]
    fn test_get_full_file_name() {
        let asset_file = AssetFile::new("XRP", "1h", 2020, 1, AgentBuilder::new().build());
        assert_eq!(asset_file.get_full_file_name(".csv"), "XRPUSDT-1h-2020-01.csv");
    }

    #[test]
    fn test_get_download_url() {
        let asset_file = AssetFile::new("EOS", "1m", 2023, 6, AgentBuilder::new().build());
        assert_eq!(asset_file.get_download_url(".zip"), "https://data.binance.vision/data/spot/monthly/klines/EOSUSDT/1m/EOSUSDT-1m-2023-06.zip");
    }

    #[test]
    fn test_get_cache_directory() {
        assert_eq!(AssetFile::get_cache_directory(), "./binance_data/downloads/");
    }

    #[test]
    fn test_get_time() {
        let asset_file = AssetFile::new("BTC", "1h", 2024, 3, AgentBuilder::new().build());
        assert_eq!(asset_file.get_time(), (3, 2024));
    }
}