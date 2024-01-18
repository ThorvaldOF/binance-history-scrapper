use std::{io};

pub struct Settings {
    pub granularity: String,
    pub asset: String,
    pub stable_coin: String,
    pub symbol: String,
    pub clear_downloads: bool,
    pub clear_extracts: bool,
}

pub fn process_input() -> Settings {
    println!("Welcome to the scrapper");
    println!("This programm uses two cache directories, [downloads] and [extracts]");
    let clear_downloads = get_clear_settings("downloads");
    let clear_extracts = get_clear_settings("extracts");

    println!("Type the granularity you want to scrap, by default it's '1s' (every second).");
    println!("Here is the list of available granularities: 1s,1m,3m,5m,15m,30m,1h,2h,4h,6h,8h,12h,1d");
    println!("Leave blank for default value: 1s");
    let granularity = get_granularity();

    println!("Type the cryptocurrency (asset) you want to scrap, BTC for example");
    let asset = get_symbol_part(true);

    println!("Type the stable coin you want to use, USDT for example");
    let stable_coin = get_symbol_part(false);
    let symbol = format!("{}{}", asset, stable_coin);

    Settings {
        granularity,
        asset,
        stable_coin,
        symbol,
        clear_downloads,
        clear_extracts,
    }
}

fn get_clear_settings(name: &str) -> bool {
    println!("Do you want to clear the [{}] directory when unused? (yes/no)", name);
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't retrieve your input, please try again");

        let input = input.trim();
        match input {
            "yes" | "y" => {
                println!("The [{}] directory will be cleared when unused", name);
                return true;
            }
            "no" | "n" => {
                println!("The [{}] directory won't be cleared when unused", name);
                return false;
            }
            _ => {}
        }
    }
}

fn get_granularity() -> String {
    let mut granularity: String = String::new();
    io::stdin().read_line(&mut granularity).expect("Couldn't retrieve your input, please try again");
    let mut granularity = granularity.trim();
    if check_granularity(&granularity) {
        println!("Granularity set to {}", granularity);
    } else {
        granularity = "1s";
        println!("Input blank or invalid, granularity set to default: 1s");
    }
    granularity.to_string()
}

fn get_symbol_part(is_asset: bool) -> String {
    let display_type = if is_asset {
        "asset"
    } else {
        "stable_coin"
    };
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Couldn't retrieve your input, please try again");
        let result = check_symbol(&input.trim(), is_asset);
        if result.is_empty() {
            println!("This {} doesn't exist, please enter a valid one", display_type);
            continue;
        } else {
            println!("Input valid, {} set to [{}]", display_type, result);
            return result;
        }
    }
}

fn check_granularity(grn: &str) -> bool {
    return match grn {
        "1s" | "1m" | "3m" | "5m" | "15m" | "30m" | "1h" | "2h" | "4h" | "6h" | "8h" | "12h" | "1d" => { true }
        _ => { false }
    };
}

fn check_symbol(constant: &str, is_asset: bool) -> String {
    let (mut asset, mut stable_coin) = ("ETH", "USDT");
    let binding = format_symbol_part(constant);
    let symbol_part = binding.as_str();
    if symbol_part.is_empty() {
        return "".to_string();
    }
    if is_asset {
        asset = symbol_part;
    } else {
        stable_coin = symbol_part;
    }

    let url = format!("https://api.binance.com/api/v3/exchangeInfo?symbol={}{}", asset, stable_coin);
    match ureq::get(&url).call() {
        Ok(res) => {
            if res.status() == 200 {
                return if is_asset {
                    asset.to_string()
                } else {
                    stable_coin.to_string()
                };
            }
        }
        Err(..) => {
            return "".to_string();
        }
    }

    "".to_string()
}

fn format_symbol_part(input: &str) -> String {
    input
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .to_uppercase()
}