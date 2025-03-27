package sql

import "database/sql"

type DB struct{ db *sql.DB }

func Open(driverName, dataSourceName string) (*DB, error) {
	var db, err = sql.Open(driverName, dataSourceName)
	return &DB{db}, err
}
func (db *DB) Begin() (*Tx, error) {
	var tx, err = db.db.Begin()
	return &Tx{tx}, err
}
func (db *DB) Close() error { return db.db.Close() }
