package os

import "syscall"

const (
	O_RDONLY int = syscall.O_RDONLY
	O_WRONLY int = syscall.O_WRONLY
	O_RDWR   int = syscall.O_RDWR
	O_CREATE int = syscall.O_CREAT
	O_EXCL   int = syscall.O_EXCL
	O_TRUNC  int = syscall.O_TRUNC
	O_APPEND int = syscall.O_APPEND
	O_SYNC   int = syscall.O_SYNC
)
const (
	PathSeparator     = '/'
	PathListSeparator = ':'
)
