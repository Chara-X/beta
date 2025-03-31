package fs

import (
	"bazil.org/fuse/fs"
	"github.com/Chara-X/go/fuse"
)

var _ fs.FS

// [fs.Server]
type Server struct{}

// [fs.New]
func New(conn *fuse.Conn, config *fs.Config) *Server

// [fs.Server.Serve]
func (s *Server) Serve(fs FS) error
