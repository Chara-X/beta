package ssh

import (
	"net"

	"golang.org/x/crypto/ssh"
)

type Conn interface {
	User() string
	SessionID() []byte
	RemoteAddr() net.Addr
	LocalAddr() net.Addr
	OpenChannel(name string, payload []byte) (net.Conn, error)
	Close() error
}

func NewClientConn(conn net.Conn, config *struct {
	User string
	Auth []ssh.AuthMethod
}) (Conn, <-chan net.Conn, <-chan *ssh.Request, error)
func NewServerConn(conn net.Conn, config *struct {
	PasswordCallback func(conn Conn, password []byte) error
}) (Conn, <-chan net.Conn, <-chan *ssh.Request, error)

// ssh.NewChannel -> ssh.Channel -> net.Conn
