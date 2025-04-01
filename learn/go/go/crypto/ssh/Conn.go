package ssh

import (
	"net"

	"golang.org/x/crypto/ssh"
)

// [ssh.Conn]
type Conn interface {
	ssh.ConnMetadata
	OpenChannel(name string, data []byte) (Channel, <-chan *Request, error)
	SendRequest(name string, wantReply bool, payload []byte) (bool, []byte, error)
	Close() error
}

// [ssh.NewClientConn]
func NewClientConn(c net.Conn, addr string, config *ssh.ClientConfig) (Conn, <-chan ssh.NewChannel, <-chan *Request, error)
