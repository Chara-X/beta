package ssh

import (
	"net"

	"golang.org/x/crypto/ssh"
)

// [ssh.Client]
type Client struct {
	Conn
}

// [ssh.Dial]
func Dial(network, addr string, config *ssh.ClientConfig) (*Client, error) {
	var c, _ = net.DialTimeout(network, addr, config.Timeout)
	var conn, _, _, err = NewClientConn(c, addr, config)
	return &Client{Conn: conn}, err
}

// [ssh.Client.NewSession]
func (c *Client) NewSession() (*Session, error) {
	var ch, reqs, _ = c.OpenChannel("session", nil)
	var s = &Session{ch: ch, exitStatus: make(chan error, 1)}
	go func() {
		<-reqs
		s.exitStatus <- nil
	}()
	return s, nil
}

// [ssh.Client.Dial]
func (c *Client) Dial(n, addr string) (net.Conn, error)
