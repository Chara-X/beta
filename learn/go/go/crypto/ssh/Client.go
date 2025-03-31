package ssh

import (
	"net"

	"github.com/Chara-X/go/go/crypto"
	"golang.org/x/crypto/ssh"
)

type Client struct {
	c *ssh.Client
	ssh.Conn
}

func Dial(network, addr string, config *ssh.ClientConfig) (*Client, error) {
	if crypto.Reference {
		var c, err = ssh.Dial(network, addr, config)
		return &Client{c: c}, err
	}
	var c, _ = net.DialTimeout(network, addr, config.Timeout)
	var conn, _, _, err = ssh.NewClientConn(c, addr, config)
	return &Client{Conn: conn}, err
}
func (c *Client) NewSession() (*Session, error) {
	if crypto.Reference {
		var s, err = c.c.NewSession()
		return &Session{s: s}, err
	}
	var ch, reqs, _ = c.OpenChannel("session", nil)
	var s = &Session{ch: ch, exitStatus: make(chan error, 1)}
	go func() {
		<-reqs
		s.exitStatus <- nil
	}()
	return s, nil
}
