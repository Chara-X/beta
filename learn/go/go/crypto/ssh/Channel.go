package ssh

import "golang.org/x/crypto/ssh"

var _ ssh.Channel

// [ssh.Channel]
type Channel interface {
	Read(data []byte) (int, error)
	Write(data []byte) (int, error)
	SendRequest(name string, wantReply bool, payload []byte) (bool, error)
	Close() error
}
