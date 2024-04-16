use std::{env, io};
use serde_json::Value;
use crate::utils::asset_file::STABLE_COIN;

const GRANULARITIES: [&str; 13] = ["1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d"];

pub struct Settings {
    pub granularity: String,
    pub assets: Vec<String>,
    pub clear_cache: bool,
}

pub fn process_input() -> Settings {
    let args: Vec<String> = env::args().collect();
    let granularity = get_flag(&args, "granularity", "1m");
    check_granularity(&granularity);

    let asset_input = get_flag(&args, "asset", "everything");
    let assets = check_asset(&asset_input);

    let clear_cache_flag = args.iter().position(|arg| arg == "clear_cache");
    let clear_cache = clear_cache_flag.is_some();

    println!("Processing on granularity: {}, assets: {} and clear_cache: {}", granularity, asset_input, clear_cache);

    Settings {
        granularity,
        assets,
        clear_cache,
    }
}

fn get_flag(args: &Vec<String>, name: &str, default: &str) -> String {
    let flag = args.iter().position(|arg| arg == name);
    if let Some(index) = flag {
        if let Some(value) = args.get(index + 1) {
            return value.to_string();
        }
    }
    default.to_string()
}

fn check_granularity(granularity: &str) {
    if !GRANULARITIES.contains(&granularity) {
        panic!("Invalid granularity, should be one of those {:?}", GRANULARITIES);
    }
}

fn check_asset(asset: &str) -> Vec<String> {
    if asset.contains("everything") {
        return get_all_assets().unwrap();
    };
    if let Some(assets) = check_symbol(asset.to_string()) {
        return vec![assets];
    };
    panic!("Invalid asset, let blank to scrap everything");
}


fn check_symbol(asset: String) -> Option<String> {
    if asset.is_empty() {
        return None;
    }
    let url = format!("https://api.binance.com/api/v3/exchangeInfo?symbol={}{}", asset, STABLE_COIN);
    match ureq::get(&url).call() {
        Ok(res) => {
            if res.status() == 200 {
                return Some(asset.to_string());
            }
        }
        _ => {
            return None;
        }
    }
    None
}

fn get_all_assets() -> Option<Vec<String>> {
    let response = ureq::get("https://api.binance.com/api/v3/exchangeInfo").call().ok()?;
    if response.status() != 200 {
        return None;
    }
    let payload = response.into_string().ok()?;

    let parsed_data: Value = serde_json::from_str(&payload).ok()?;

    let symbols_array = parsed_data.get("symbols")?.as_array()?;

    let mut asset_pairs: Vec<String> = Vec::new();
    for symbol in symbols_array {
        let quote_asset = symbol.get("quoteAsset")?.as_str()?.to_string();
        if quote_asset == STABLE_COIN {
            let base_asset = symbol.get("baseAsset")?.as_str()?.to_string();
            asset_pairs.push(base_asset);
        }
    }

    Some(asset_pairs)
}

fn format_asset(input: &mut String) {
    *input = input
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .to_uppercase();
}