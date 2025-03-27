package sql

import "database/sql"

type Rows struct{ rs *sql.Rows }

func (rs *Rows) Columns() ([]string, error) { return rs.rs.Columns() }
func (rs *Rows) Next() bool                 { return rs.rs.Next() }
func (rs *Rows) Scan(dest ...any) error     { return rs.rs.Scan(dest...) }
func (rs *Rows) Close() error               { return rs.rs.Close() }
