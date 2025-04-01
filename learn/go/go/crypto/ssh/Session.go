package ssh

import (
	"io"

	"golang.org/x/crypto/ssh"
)

// [ssh.Session]
type Session struct {
	ch         Channel
	exitStatus chan error
	Stdin      io.Reader
	Stdout     io.Writer
}

// [ssh.Session.StdinPipe]
func (s *Session) StdinPipe() (io.WriteCloser, error) {
	return s.ch, nil
}

// [ssh.Session.StdoutPipe]
func (s *Session) StdoutPipe() (io.Reader, error) {
	return s.ch, nil
}

// [ssh.Session.RequestPty]
func (s *Session) RequestPty(term string, h, w int, termmodes ssh.TerminalModes) error

// [ssh.Session.Setenv]
func (s *Session) Setenv(name, value string) error

// [ssh.Session.Shell]
func (s *Session) Shell() error

// [ssh.Session.Start]
func (s *Session) Start(cmd string) error {
	var _, err = s.ch.SendRequest("exec", true, ssh.Marshal(&struct{ Command string }{Command: cmd}))
	return err
}

// [ssh.Session.Wait]
func (s *Session) Wait() error {
	return <-s.exitStatus
}

// [ssh.Session.SendRequest]
func (s *Session) SendRequest(name string, wantReply bool, payload []byte) (bool, error)

// [ssh.Session.Close]
func (s *Session) Close() error {
	return s.ch.Close()
}
