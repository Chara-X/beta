package bolt

import (
	"os"

	"github.com/boltdb/bolt"
)

type DB struct {
	StrictMode bool
	NoSync     bool
	NoGrowSync bool
	MmapFlags  int
	AllocSize  int
	// ...
}

func Open(path string, mode os.FileMode, options *bolt.Options) (*DB, error)
func (db *DB) Begin(writable bool) (*Tx, error)
func (db *DB) Close() error
