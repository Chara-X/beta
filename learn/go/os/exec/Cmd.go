package exec

import (
	"context"
	"io"
	"os"
	"syscall"
)

type Cmd struct {
	Path         string
	Args         []string
	Env          []string
	Dir          string
	Stdin        io.Reader
	Stdout       io.Writer
	Stderr       io.Writer
	SysProcAttr  *syscall.SysProcAttr
	ProcessState *os.ProcessState
	Cancel       func() error
}

func CommandContext(ctx context.Context, name string, arg ...string) *Cmd
func (cmd *Cmd) StdinPipe() (io.WriteCloser, error)
func (cmd *Cmd) StdoutPipe() (io.ReadCloser, error)
func (cmd *Cmd) StderrPipe() (io.ReadCloser, error)
func (cmd *Cmd) Run() error
func (cmd *Cmd) Start() error
func (cmd *Cmd) Wait() error
