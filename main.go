package main

import (
	"crypto-history-scrapper/utils"
	"fmt"
	"os"
)

func main() {
	var cryptos = processInput()
	allProcesses := make(map[string]*cryptoProcess)
	utils.Info("Start download")
	for i := 0; i < len(cryptos); i++ {
		utils.Info(fmt.Sprintf("Starting %v's data download", cryptos[i]))
		tempProcess := cryptoProcess{count: 0, rc: make(chan error)}
		intervalCounter := 0
	DownloadsLoop:
		for currentYear := todayYear; currentYear >= binanceBirth; currentYear-- {
			maxMonth := 12
			if currentYear == todayYear {
				maxMonth = todayMonth
			}
			for currentMonth := maxMonth; currentMonth >= 1; currentMonth-- {
				currentCrypto := cryptoFile{
					name:  cryptos[i],
					year:  currentYear,
					month: currentMonth,
				}
				err := downloadFile(&currentCrypto)
				if err != nil {
					if err.Error() == "no file found" {
						utils.Success(fmt.Sprintf("Download of %v finished, no data available before %v/%v (included)", currentCrypto.name, currentCrypto.month, currentCrypto.year))
						break DownloadsLoop
					}
				}
				tempProcess.count++
				go extractFile(currentCrypto, &tempProcess, &intervalCounter)
			}
		}
		allProcesses[cryptos[i]] = &tempProcess
	}
	utils.Success("Finished Downloads, processing extraction")
	mergeProcesses := cryptoProcess{count: len(allProcesses), rc: make(chan error)}
	for index, _ := range allProcesses {
		waitProcesses(allProcesses[index])
		go mergeResults(index, &mergeProcesses)
		utils.Info(" Extraction finished for " + index + " start merging it")
	}
	waitProcesses(&mergeProcesses)
	clearCache()
	utils.Success("Merge finished")
	utils.Info("Scrapping completed, you can find your output in 'results' directory")
	utils.Log("Press enter to quit...")
	fmt.Scanln()
}

func waitProcesses(cp *cryptoProcess) {
	for i := 0; i < cp.count; i++ {
		<-cp.rc
	}
	close(cp.rc)
	for err := range cp.rc {
		utils.Error(err.Error())
	}
}
func clearCache() {
	if clearDownloads {
		err := os.RemoveAll(downloadsPath)
		if err != nil {
			utils.Error(err.Error())
		}
	}
	if clearExtracts {
		err := os.RemoveAll(extractsPath)
		if err != nil {
			utils.Error(err.Error())
		}
	}
}
