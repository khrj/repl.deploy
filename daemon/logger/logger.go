package logger

import (
	"fmt"
	"os"
	"time"

	"github.com/fatih/color"
)

func Success(s string) {
	color.Green(padStrWithTime(s))
}

func Info(s string) {
	color.Blue(padStrWithTime(s))
}

func Warn(s string) {
	color.Yellow(padStrWithTime(s))
}

func Error(s string) {
	color.Red(padStrWithTime(s))
}

func FatalError(s string) {
	Error(s)
	os.Exit(0)
}

func padStrWithTime(toPad string) string {
	currentTime := time.Now().UTC()

	year, month, day := currentTime.Date()
	hour, min, sec := currentTime.Clock()

	return fmt.Sprintf("%04d/%02d/%02d %02d:%02d:%02d %s", year, int(month), day, hour, min, sec, toPad)
}
