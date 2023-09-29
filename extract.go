package main

import (
	"archive/zip"
	"encoding/csv"
	"fmt"
	"io"
	"os"
)

func extractFile(currentCrypto cryptoFile, cp *cryptoProcess, intervalCounter *int) {
	filedir := fmt.Sprintf("%v%v/", extractsPath, currentCrypto.name)
	filename := currentCrypto.file + ".csv"
	sourceFile := downloadsPath + currentCrypto.name + "/" + currentCrypto.file + ".zip"

	archive, err := zip.OpenReader(sourceFile)
	cp.handle(err, "Error opening source zip archive "+sourceFile)
	defer func(archive *zip.ReadCloser) {
		cp.handle(archive.Close(), "Error closing source zip archive "+sourceFile)
		if clearDownloads {
			cp.handle(os.Remove(sourceFile), "Error deleting source zip archive "+sourceFile)
		}
	}(archive)

	file := archive.Reader.File[0]
	reader, err := file.Open()
	cp.handle(err, "Error reading source zip archive "+sourceFile)
	defer func(reader io.ReadCloser) {
		cp.handle(reader.Close(), "Error closing source zip archive "+sourceFile)
	}(reader)

	path := filedir + filename
	_ = os.Remove(path)
	cp.handle(os.MkdirAll(path, os.ModePerm), "Error creating extraction folder "+path)
	cp.handle(os.Remove(path), "Error deleting extraction folder "+path)

	writer, err := os.OpenFile(path, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, file.Mode())
	cp.handle(err, "Error opening target file "+path)
	defer func(writer *os.File) {
		cp.handle(writer.Close(), "Error closing target file "+path)
	}(writer)

	csvWriter := csv.NewWriter(writer)
	defer csvWriter.Flush()

	csvReader := csv.NewReader(reader)
	for {
		record, err := csvReader.Read()
		if err == io.EOF {
			break
		}
		cp.handle(err, "Error reading CSV record from source zip archive "+sourceFile)

		if !skipLine(intervalCounter) {
			cp.handle(csvWriter.Write(record), "Error writing CSV record to target file "+path)
		}
	}
}

func skipLine(count *int) bool {
	*count++
	if *count >= interval {
		*count = 0
		return false
	}
	return true
}
