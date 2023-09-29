package utils

import (
	"fmt"
	"github.com/fatih/color"
	"log"
)

var (
	DebugMode bool
)

func Debug(err error, ctx string) error {
	msg := ctx
	if DebugMode {
		msg += " -> " + err.Error()
	}
	log.Println(color.RedString("%v", msg))

	return err
}
func Error(msg string) {
	fmt.Println(color.RedString("%v", msg))
}

func Warning(msg string) {
	fmt.Println(color.YellowString("%v", msg))
}

func Info(msg string) {
	fmt.Println(color.CyanString("%v", msg))
}

func Log(msg string) {
	fmt.Println(msg)
}

func Success(msg string) {
	fmt.Println(color.GreenString("%v", msg))
}
