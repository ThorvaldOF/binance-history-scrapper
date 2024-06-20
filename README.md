# Crypto History Scrapper

This Rust program retrieves historical data for a cryptocurrency from the Binance API.

## Usage Instructions

In order to use the program, you need to use flags, here's the syntax

`./[program_name] granularity [value] asset [value] clear_cache`

1. **Granularity**

   Here is the list of available granularities:
    - 1s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d
   
   Syntax example :`./[program_name] granularity 12h`
   The default value is `1m`

2. **Asset**
   
   A check is made when selecting an asset, if it's available on Binance, it should work

   Syntax example :`./[program_name] asset BTC`
   The default value is `everything`, which means that the program will scrap all assets available on Binance

3. **Clear cache**

   When processing, the program will create a `downloads` directory, containing all the `.zip` and `.CHECKSUM` files.
   By entering the `clear_cache` flag, the program will clear this directory in real time, saving you disk storage.

   By default, this feature is off.

## Output

Once the program completes, the results will be available in the 'results' directory.

## Note

This program uses the Binance API to validate cryptocurrency names. Ensure you have an active internet connection during
execution.
