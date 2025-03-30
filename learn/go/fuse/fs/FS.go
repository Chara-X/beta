package fs

import "bazil.org/fuse/fs"

var _ fs.FS

// [fs.FS]
type FS interface {
	Root() (Node, error)
}
