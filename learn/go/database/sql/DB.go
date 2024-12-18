package sql

type DB struct{}

func Open(driverName, dataSourceName string) (*DB, error)
func (db *DB) Begin() (*Tx, error)
func (db *DB) Query(query string, args ...any) (*Rows, error)
func (db *DB) Close() error
