package procfs

import "github.com/prometheus/procfs"

type Proc struct{ PID int }

func (p Proc) Stat() (procfs.ProcStat, error)
func (p Proc) Comm() (string, error)
func (p Proc) CmdLine() ([]string, error)
func (p Proc) Environ() ([]string, error)
func (p Proc) RootDir() (string, error)
func (p Proc) Cwd() (string, error)
func (p Proc) FileDescriptors() ([]uintptr, error)

// ...
