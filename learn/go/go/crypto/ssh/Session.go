package ssh

import (
	"io"

	"github.com/Chara-X/go-study/go/crypto"
	"golang.org/x/crypto/ssh"
)

type Session struct {
	s          *ssh.Session
	ch         ssh.Channel
	exitStatus chan error
}

func (s *Session) StdinPipe() (io.WriteCloser, error) {
	if crypto.Reference {
		return s.s.StdinPipe()
	}
	return s.ch, nil
}
func (s *Session) StdoutPipe() (io.Reader, error) {
	if crypto.Reference {
		return s.s.StdoutPipe()
	}
	return s.ch, nil
}
func (s *Session) Start(cmd string) error {
	if crypto.Reference {
		return s.s.Start(cmd)
	}
	var _, err = s.ch.SendRequest("exec", true, ssh.Marshal(&struct{ Command string }{Command: cmd}))
	return err
}
func (s *Session) Wait() error {
	if crypto.Reference {
		return s.s.Wait()
	}
	return <-s.exitStatus
}
func (s *Session) Close() error {
	if crypto.Reference {
		return s.s.Close()
	}
	return s.ch.Close()
}
