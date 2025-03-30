package fuse

import "bazil.org/fuse"

var _ fuse.Conn

// [fuse.Conn]
type Conn struct{}

// [fuse.Mount]
func Mount(dir string, options ...fuse.MountOption) (*Conn, error)

// [fuse.Conn.Close]
func (c *Conn) Close() error
