package sql

import "database/sql"

type Rows struct{}

func (rows *Rows) ColumnTypes() ([]*sql.ColumnType, error)
func (rows *Rows) Next() bool
func (rows *Rows) Scan(dest ...any) error
func (rows *Rows) Close() error
