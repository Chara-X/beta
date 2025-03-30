package fuse

import (
	"os/exec"

	"bazil.org/fuse"
)

var _ = fuse.Unmount

// [fuse.Unmount]
func Unmount(dir string) error {
	return exec.Command("fusermount3", "-u", dir).Run()
}
