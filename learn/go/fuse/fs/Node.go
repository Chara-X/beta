package fs

import (
	"context"

	"bazil.org/fuse"
	"bazil.org/fuse/fs"
)

var _ fs.Node

// [fs.Node]
type Node interface {
	Attr(ctx context.Context, attr *fuse.Attr) error
}

// [fs.NodeStringLookuper]
type NodeStringLookuper interface {
	Lookup(ctx context.Context, name string) (Node, error)
}

// [fs.NodeRenamer]
type NodeRenamer interface {
	Rename(ctx context.Context, req *fuse.RenameRequest, newDir Node) error
}

// [fs.NodeRemover]
type NodeRemover interface {
	Remove(ctx context.Context, req *fuse.RemoveRequest) error
}

// [fs.NodeOpener]
type NodeOpener interface {
	Open(ctx context.Context, req *fuse.OpenRequest, resp *fuse.OpenResponse) (Handle, error)
}
