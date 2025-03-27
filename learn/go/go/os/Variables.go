package os

import (
	"os"
	"syscall"
)

var Args []string
var (
	Stdin  = os.NewFile(uintptr(syscall.Stdin), "/dev/stdin")
	Stdout = os.NewFile(uintptr(syscall.Stdout), "/dev/stdout")
	Stderr = os.NewFile(uintptr(syscall.Stderr), "/dev/stderr")
)
