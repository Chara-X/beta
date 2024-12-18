package sql

type Null[T any] struct {
	V     T
	Valid bool
}

func (null *Null[T]) Scan(src any) error
