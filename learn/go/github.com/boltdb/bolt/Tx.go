package bolt

import "io"

type Tx struct {
	Root      Bucket
	WriteFlag int
}

func (tx *Tx) ID() int
func (tx *Tx) Commit() error
func (tx *Tx) Rollback() error
func (tx *Tx) WriteTo(w io.Writer) (int64, error)

// Root: root, CreateBucket, DeleteBucket, Bucket, ...
