package sql

type Tx struct{}

func (tx *Tx) Query(query string, args ...any) (*Rows, error)
func (tx *Tx) Commit() error
func (tx *Tx) Rollback() error
