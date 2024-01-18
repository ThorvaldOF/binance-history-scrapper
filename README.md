# Crypto History Scrapper

This Rust program retrieves historical data for a cryptocurrency from the Binance API.

## Usage Instructions

1. **Granularity**

   When launching the program, you can set the data retrieval granularity, by default set to '1s' (every second).

   Here is the list of available granularities:
    - 1s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d

   Input format: `[Granularity]`. Leave it blank to use default settings.

2. **Cryptocurrency to retrieve (asset)**
   Enter the names of the asset you want to retrieve. The program validates names using the Binance API.

   While the asset isn't valid you can't continue.

3. **Stable coin to use**

In combination of the asset you need a stable coin, the process is more or less the same as the asset retrieval

## Output

Once the program completes, the results will be available in the 'results' directory. Temporary download files are
automatically deleted.

**Note:** Press Enter to exit the program after execution.

## Note

This program uses the Binance API to validate cryptocurrency names. Ensure you have an active internet connection during
execution.
