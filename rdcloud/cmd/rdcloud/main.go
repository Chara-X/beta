package main

import (
	"fmt"

	"github.com/Chara-X/rdcloud"
)

func main() {
	var db = rdcloud.New("rdcloud.db")
	// db.ImportTroublesFromCSV("trouble.csv")
	for _, trouble := range db.QueryAllTroubles() {
		for k, v := range trouble {
			fmt.Printf("%s: %s\n", k, v)
		}
		fmt.Println("=====================")
	}
}
