package pkg

import (
	"os"
	"path/filepath"
)

var GlobalEnv = func() string {
	e := os.Getenv("ZETTL_ENV")
	if e != "development" && e != "production" {
		e = "production"
	}
	return e
}()

func GetDataDir(env string) string {
	if env == "development" {
		wd, err := os.Getwd()
		if err != nil {
			return "data"
		}
		return filepath.Join(wd, "data")
	} else {
		homeDir, err := os.UserHomeDir()
		if err != nil {
			panic(err)
		}
		return filepath.Join(homeDir, "zettl")
	}
}
