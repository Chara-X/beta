// [embed]
package embed

import (
	"embed"
	"io/fs"
)

var _ embed.FS

type FS struct{}

// [embed.FS.Open]
func (f FS) Open(name string) (fs.File, error)

// [embed.FS.ReadDir]
func (f FS) ReadDir(name string) ([]fs.DirEntry, error)

// [embed.FS.ReadFile]
func (f FS) ReadFile(name string) ([]byte, error)
