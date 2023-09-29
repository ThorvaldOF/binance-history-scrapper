package main

import (
	"bufio"
	"crypto-history-scrapper/utils"
	"flag"
	"fmt"
	"net/http"
	"os"
	"strconv"
	"strings"
)

func processInput() []string {
	getFlags()
	reader := bufio.NewReader(os.Stdin)
	utils.Success("Welcome to the scrapper")
	utils.Log("This programm uses two cache directories, [downloads] and [extracts]")
	getClearParameter("downloads", &clearDownloads, reader)
	getClearParameter("extracts", &clearExtracts, reader)
	clearCache()

	utils.Info("Type the granularity you want to scrap, by default it's '1s' (every second).")
	utils.Log("Here is the list of available granularities: 1s,1m,3m,5m,15m,30m,1h,2h,4h,6h,8h,12h,1d")
	utils.Info("You can also set an scrapping interval to aggregate the data of a granularity")
	utils.Log("For example you can set '10' in combination with '1s' to get the aggregated data from every 10 seconds")
	utils.Log("Here is the format [Granularity] [Interval], leave blank for default value")
	fmt.Scanf("%v %d", &granularity, &interval)
	if !checkGranularity(granularity) {
		utils.Warning("Input blank or invalid, loaded default settings : 1s 0")
	} else {
		utils.Info("Parameters set to " + granularity + " " + strconv.Itoa(interval))
	}
	utils.Info("Type the crypto you want to scrap")
	var cryptoCurrencies []string
	for {
		var tempCrypto string
		_, err := fmt.Scanln(&tempCrypto)
		if err != nil && tempCrypto != "" {
			utils.Error("Invalid input, please try again")
			continue
		}
		tempCrypto = checkCryptoName(tempCrypto)
		if tempCrypto == "INVALID" {
			utils.Warning("This crypto doesn't exist, please enter a valid crypto or let blank to continue")
		} else if tempCrypto == "" {
			if len(cryptoCurrencies) > 0 {
				break
			} else {
				utils.Warning("Enter at least one crypto currency")
			}
		} else {
			cryptoCurrencies = append(cryptoCurrencies, tempCrypto)
			utils.Success("[" + tempCrypto + "] added, you can add another crypto if you want, or leave the field blank to start the process")
		}
	}
	return cryptoCurrencies
}

func checkGranularity(grn string) bool {
	switch grn {
	case "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d":
		return true
	default:
		return false
	}
}

func checkCryptoName(name string) string {
	if name == "" {
		return ""
	}
	name = strings.ReplaceAll(name, " ", "")
	name = strings.ToUpper(name)
	url := fmt.Sprintf("https://api.binance.com/api/v3/exchangeInfo?symbol=%s%v", name, stableCoin)
	response, err := http.Get(url)
	if err != nil {
		return "INVALID"
	}
	defer response.Body.Close()
	if response.StatusCode != http.StatusOK {
		return "INVALID"
	}
	return name
}

func getFlags() {
	flag.BoolVar(&utils.DebugMode, "debug", false, "display debug logs")
	flag.Parse()
}

func getClearParameter(name string, variable *bool, reader *bufio.Reader) {
	utils.Info("Do you want to clear the [" + name + "] directory when unused? (yes/no)")
param:
	for {
		res, _ := reader.ReadString('\n')
		res = strings.TrimSpace(res)
		switch strings.ToLower(res) {
		case "no", "n":
			*variable = false
			utils.Log("The [" + name + "] won't be cleared when unused")
			break param
		case "yes", "y":
			*variable = true
			utils.Log("The [" + name + "] will be cleared when unused")
			break param
		default:
			utils.Error("Expected yes or no, please try again")
		}
	}
}
