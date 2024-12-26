package rdcloud

import (
	"database/sql"
	"encoding/csv"
	"os"
	"strings"

	_ "modernc.org/sqlite"
)

const (
	createTroubles = `CREATE TABLE troubles (
		标识 TEXT PRIMARY KEY NOT NULL, 
		标题 TEXT,
		工作项类型 TEXT,
		状态 TEXT,
		变更大类 TEXT,
		缺陷等级 TEXT,
		创建时间 TEXT,
		创建人部门 TEXT,
		iChange发现版本号 TEXT,
		发现活动 TEXT,
		引入活动 TEXT,
		缺陷来源 TEXT,
		版本所处阶段 TEXT,
		引入者 TEXT,
		缺陷位置 TEXT,
		故障引入年份 TEXT,
		排查标识 TEXT,
		FOREIGN KEY(排查标识) REFERENCES shoots(标识) 
	)`
	createShoots = `CREATE TABLE shoots (
		标识 TEXT PRIMARY KEY NOT NULL,
		排查手段 TEXT,
		手段类别 TEXT,
	)`
)

type DB struct{ *sql.DB }

func Open(dataSourceName string) *DB {
	var db, err = sql.Open("sqlite", dataSourceName)
	if err != nil {
		panic(err)
	}
	return &DB{db}
}
func (db *DB) CreateTroubles() {
	if _, err := db.Exec(createTroubles); err != nil {
		panic(err)
	}
}
func (db *DB) CreateShoots() {
	if _, err := db.Exec(createShoots); err != nil {
		panic(err)
	}
}
func (db *DB) InsertCSV(table, csvPath string) {
	var f, err = os.Open(csvPath)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	var r = csv.NewReader(f)
	cols, err := r.Read()
	if err != nil {
		panic(err)
	}
	var rows = []string{}
	for row, err := r.Read(); err == nil; row, err = r.Read() {
		rows = append(rows, "('"+strings.Join(row, "','")+"')")
	}
	if _, err := db.Exec("INSERT INTO " + table + " (" + strings.Join(cols, ",") + ") VALUES " + strings.Join(rows, ",")); err != nil {
		panic(err)
	}
}
func (db *DB) AllTroubles() *sql.Rows {
	var rows, err = db.Query("SELECT * FROM troubles")
	if err != nil {
		panic(err)
	}
	return rows
}
