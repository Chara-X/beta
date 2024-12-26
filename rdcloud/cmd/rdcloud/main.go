package main

import (
	"github.com/Chara-X/rdcloud"
	_ "modernc.org/sqlite"
)

// rdcloud new trouble()
func main() {
	var db = rdcloud.Open("rdcloud.db")
	db.CreateTroubles()
	db.InsertCSV("troubles", "troubles.csv")
}
