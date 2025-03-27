package sql

import "database/sql"

type Null[T any] struct {
	n     *sql.Null[T]
	V     T
	Valid bool
}

func (n *Null[T]) Scan(value any) error { return n.n.Scan(value) }
