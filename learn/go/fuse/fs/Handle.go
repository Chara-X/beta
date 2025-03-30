package fs

import (
	"context"

	"bazil.org/fuse"
	"bazil.org/fuse/fs"
)

var _ fs.Handle

// [fs.Handle]
type Handle any

// [fs.HandleReadDirAller]
type HandleReadDirAller interface {
	ReadDirAll(ctx context.Context) ([]fuse.Dirent, error)
}

// [fs.HandleReader]
type HandleReader interface {
	Read(ctx context.Context, req *fuse.ReadRequest, resp *fuse.ReadResponse) error
}

// [fs.HandleWriter]
type HandleWriter interface {
	Write(ctx context.Context, req *fuse.WriteRequest, resp *fuse.WriteResponse) error
}

// [fs.HandleFlusher]
type HandleFlusher interface {
	Flush(ctx context.Context, req *fuse.FlushRequest) error
}

// [fs.HandleReleaser]
type HandleReleaser interface {
	Release(ctx context.Context, req *fuse.ReleaseRequest) error
}
