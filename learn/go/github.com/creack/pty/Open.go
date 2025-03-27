package pty

import "os"

func Open() (pty, tty *os.File, err error)
