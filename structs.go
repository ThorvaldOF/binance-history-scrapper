package main

import (
	"crypto-history-scrapper/utils"
	"time"
)

type cryptoFile struct {
	name  string
	year  int
	month int
	file  string
}

type cryptoProcess struct {
	count int
	rc    chan error
}

func (cp *cryptoProcess) handle(err error, ctx string) {
	if err != nil {
		cp.rc <- utils.Debug(err, ctx)
	}
}

const (
	binanceBirth  = 2017
	downloadsPath = "./downloads/"
	extractsPath  = "./extracts/"
	resultsPath   = "./results/"
	stableCoin    = "USDT"
)

var (
	today          = time.Now().AddDate(0, -1, 0)
	todayYear      = today.Year()
	todayMonth     = int(today.Month())
	granularity    = "1s"
	interval       = 0
	clearDownloads = true
	clearExtracts  = true
)
