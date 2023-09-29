# Crypto History Scrapper

This Go program retrieves historical data for different cryptocurrencies from the Binance API.

## Usage Instructions

1. **Granularity and Interval Settings**

   When launching the program, you can set the data retrieval granularity, by default set to '1s' (every second). You can also specify a retrieval interval to aggregate data for the chosen granularity. For example, setting '10' with '1s' aggregates data every 10 seconds.

   Here is the list of available granularities:
    - 1s, 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 6h, 8h, 12h, 1d

   Input format: `[Granularity] [Interval]`. Leave it blank to use default settings.

2. **Cryptocurrencies to Retrieve**

   Enter the names of the cryptocurrencies you want to retrieve. The program validates names using the Binance API.

   If a cryptocurrency name is invalid, it will be ignored. You can leave the field blank to start the process if at least one cryptocurrency has been added.


## Output

Once the program completes, the results will be available in the 'results' directory. Temporary download files are automatically deleted.

**Note:** Press Enter to exit the program after execution.

## Note

This program uses the Binance API to validate cryptocurrency names. Ensure you have an active internet connection during execution.
