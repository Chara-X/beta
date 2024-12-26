package sql

import "database/sql"

type Tx struct{ tx *sql.Tx }

func (tx *Tx) Query(query string, args ...any) (*Rows, error) {
	var rows, err = tx.tx.Query(query, args...)
	return &Rows{rows}, err
}
func (tx *Tx) Commit() error   { return tx.tx.Commit() }
func (tx *Tx) Rollback() error { return tx.tx.Rollback() }
