package main

import (
	"crypto-history-scrapper/utils"
	"errors"
	"fmt"
	"github.com/cavaliergopher/grab/v3"
	"github.com/fatih/color"
	"os"
	"strings"
	"time"
)

func downloadFile(currentCrypto *cryptoFile) (err error) {
	monthPrefix := ""
	if currentCrypto.month < 10 {
		monthPrefix = "0"
	}
	filedir := fmt.Sprintf("%v%v/", downloadsPath, currentCrypto.name)
	currentCrypto.file = fmt.Sprintf("%v%v-%v-%v-%v%v", currentCrypto.name, stableCoin, granularity, currentCrypto.year, monthPrefix, currentCrypto.month)
	currentCryptoFile := currentCrypto.file + ".zip"
	url := fmt.Sprintf("https://data.binance.vision/data/spot/monthly/klines/%v%v/%v/%v", currentCrypto.name, stableCoin, granularity, currentCryptoFile)

	// Creating folder
	if _, err := os.Stat(filedir); errors.Is(err, os.ErrNotExist) {
		err := os.Mkdir(filedir, os.ModePerm)
		if err != nil {
			err = os.MkdirAll(filedir, os.ModePerm)
		}
	}

	// Starting request
	client := grab.NewClient()
	req, _ := grab.NewRequest(filedir, url)
	resp := client.Do(req)

	var downloadMessage string
	switch resp.HTTPResponse.StatusCode {
	case 200:
		downloadMessage = fmt.Sprintf("Downloading %v...", currentCrypto.file)
	case 206:
		downloadMessage = fmt.Sprintf("Resuming %v...", currentCrypto.file)
	default:
		return errors.New("no file found")
	}

	t := time.NewTicker(100 * time.Millisecond)
	defer t.Stop()

DisplayLoop:
	for {
		select {
		case <-t.C:
			fmt.Printf("\r %v %v (%.2f%%)",
				downloadMessage, percentToBar(int(100*resp.Progress())), 100*resp.Progress(),
			)
		case <-resp.Done:
			fmt.Printf(color.GreenString("\r %v Downloaded successfully !%v\n"),
				currentCrypto.file, successBar())
			break DisplayLoop
		}
	}

	if err := resp.Err(); err != nil {
		utils.Error("File " + currentCrypto.file + " failed")
	}
	return nil
}

func percentToBar(percent int) string {
	bar := strings.Builder{}
	bar.WriteRune('[')
	bar.WriteString(strings.Repeat(string('='), percent))
	bar.WriteRune('>')
	bar.WriteString(strings.Repeat(string(' '), 100-percent))
	bar.WriteRune(']')
	return bar.String()
}
func successBar() string {
	return strings.Repeat(" ", 120)
}
