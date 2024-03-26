use std::{io};
use serde_json::Value;
use crate::utils::asset_file::STABLE_COIN;

const GRANULARITIES: [&str; 13] = ["1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d"];

pub struct Settings {
    pub granularity: String,
    pub assets: Vec<String>,
    pub clear_cache: bool,
}

pub fn process_input() -> Settings {
    println!("Welcome to the scrapper");

    println!("Type the granularity you want to scrap, by default it's '1s' (every second).");
    println!("Here is the list of available granularities: {:?}", GRANULARITIES);
    println!("Leave blank for default value: 1s");
    let granularity = get_granularity();

    println!("Type the asset you want to scrap, BTC for example. Type EVERYTHING to crap all available assets");
    let assets = get_asset();

    println!("Do you want to clear the [downloads] directory when unused? (yes/no)");
    let clear_cache = get_clear_cache();

    Settings {
        granularity,
        assets,
        clear_cache,
    }
}

fn get_granularity() -> String {
    let mut granularity: String = String::new();
    loop {
        granularity.clear();
        io::stdin().read_line(&mut granularity).expect("Couldn't retrieve your input, please try again");
        let granularity = granularity.trim();
        if GRANULARITIES.contains(&granularity) {
            println!("Granularity set to {}", granularity);
            return granularity.to_string();
        } else {
            println!("Input blank or invalid, please enter a valid granularity");
            println!("Reminder, here is the list of available granularities: {:?}", GRANULARITIES);
        }
    }
}

fn get_asset() -> Vec<String> {
    let mut assets: Vec<String> = Vec::new();
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't retrieve your input, please try again");
        format_asset(&mut input);
        if input == "EVERYTHING" {
            match get_all_assets() {
                Some(all_assets) => {
                    println!("Okay, we will processing all available assets on Binance, here is the list: {:?}", all_assets);
                    return all_assets;
                }
                None => {
                    println!("An error occured while fetching all available assets");
                    continue;
                }
            }
        }
        let result = check_symbol(input);
        match result {
            Some(asset) => {
                println!("Input valid, asset set to [{}]", asset);
                assets.push(asset);
                return assets;
            }
            None => {
                println!("This asset doesn't exist, please enter a valid one");
            }
        }
    }
}

fn get_clear_cache() -> bool {
    let mut input: String = String::new();
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Couldn't retrieve your input, please try again");

        let input = input.trim();
        match input {
            "yes" | "y" => {
                println!("The [downloads] directory will be cleared when unused");
                return true;
            }
            "no" | "n" => {
                println!("The [downloads] directory won't be cleared when unused");
                return false;
            }
            _ => {
                println!("Please answer by yes or no, (y and n also works)");
            }
        }
    }
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