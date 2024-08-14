use std::{env};
use serde_json::Value;
use crate::utils::asset_file::STABLE_COIN;

pub const GRANULARITIES: [(&str, u64); 13] = [
    ("1s", 1_000),
    ("1m", 60_000),
    ("3m", 3 * 60_000),
    ("5m", 5 * 60_000),
    ("15m", 15 * 60_000),
    ("30m", 30 * 60_000),
    ("1h", 60 * 60_000),
    ("2h", 2 * 60 * 60_000),
    ("4h", 4 * 60 * 60_000),
    ("6h", 6 * 60 * 60_000),
    ("8h", 8 * 60 * 60_000),
    ("12h", 12 * 60 * 60_000),
    ("1d", 24 * 60 * 60_000)];

pub struct Settings {
    pub granularity: String,
    pub assets: Vec<String>,
}

pub fn process_input() -> Settings {
    let args: Vec<String> = env::args().collect();
    let granularity = get_flag(&args, "granularity", "1m");
    check_granularity(&granularity);

    let asset_input = get_flag(&args, "asset", "everything");
    let assets = check_asset(&asset_input);


    println!("Processing on granularity: {} and assets: {}, should we continue ? (Y/n)", granularity, asset_input);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "Y" && input.trim() != "y" {
        panic!("Exiting");
    }

    Settings {
        granularity,
        assets,
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
    if !GRANULARITIES.iter().any(|&(key, _)| key == granularity) {
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
        let status = symbol.get("status")?.as_str()?.to_string();
        if status != "TRADING" {
            continue;
        }
        let quote_asset = symbol.get("quoteAsset")?.as_str()?.to_string();
        if quote_asset == STABLE_COIN {
            let base_asset = symbol.get("baseAsset")?.as_str()?.to_string();
            asset_pairs.push(base_asset);
        }
    }

    Some(asset_pairs)
}