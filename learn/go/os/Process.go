package os

import "os"

type Process struct {
	Pid int
}

func StartProcess(name string, args []string, attributes *os.ProcAttr) (*Process, error)
func FindProcess(pid int) (*Process, error)
func (process *Process) Signal(signal os.Signal) error
func (process *Process) Wait() (*os.ProcessState, error)
