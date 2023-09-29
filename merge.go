package main

import (
	"encoding/csv"
	"errors"
	"os"
	"sort"
)

func mergeResults(cryptoCurrency string, cp *cryptoProcess) {
	sourcePath := extractsPath + cryptoCurrency + "/"
	targetFilePath := resultsPath + cryptoCurrency + stableCoin + ".csv"
	dirFiles, err := os.Open(sourcePath)
	cp.handle(err, "Error opening extraction file "+cryptoCurrency)
	defer func(dirFiles *os.File) {
		cp.handle(dirFiles.Close(), "Error closing extraction folder "+cryptoCurrency)
	}(dirFiles)
	files, err := dirFiles.Readdir(0)
	cp.handle(err, "Error reading extraction file "+cryptoCurrency)
	sort.Slice(files, func(i, j int) bool {
		return files[i].Name() < files[j].Name()
	})

	for _, sourceFile := range files {
		func(sourceFile os.FileInfo) {
			f, err := os.Open(sourcePath + sourceFile.Name())
			cp.handle(err, "Error opening source file "+sourceFile.Name())
			defer func(f *os.File, sp, sf string) {
				cp.handle(f.Close(), "Error closing source file "+sf)
				if clearExtracts {
					cp.handle(os.Remove(sp+sf), "Error deleting source file "+sf)
				}
			}(f, sourcePath, sourceFile.Name())

			csvReader := csv.NewReader(f)
			records, err := csvReader.ReadAll()
			cp.handle(err, "Error parsing CSV source file "+sourceFile.Name())

			// Creating folder
			if _, err := os.Stat(resultsPath); errors.Is(err, os.ErrNotExist) {
				err := os.Mkdir(resultsPath, os.ModePerm)
				if err != nil {
					err = os.MkdirAll(resultsPath, os.ModePerm)
				}
			}

			if _, err := os.Stat(targetFilePath); errors.Is(err, os.ErrNotExist) {
				_, err := os.Create(targetFilePath)
				cp.handle(err, "Error creating target file "+targetFilePath)
			}

			targetFile, err := os.OpenFile(targetFilePath, os.O_WRONLY|os.O_CREATE|os.O_APPEND, 0644)
			cp.handle(err, "Error opening target file "+targetFilePath)
			defer func(targetFile *os.File) {
				cp.handle(targetFile.Close(), "Error closing target file "+targetFilePath)
			}(targetFile)

			w := csv.NewWriter(targetFile)
			cp.handle(w.WriteAll(records), "Error writing target file "+targetFilePath)
		}(sourceFile)
	}
	cp.rc <- nil
}
