package ssh

import "golang.org/x/crypto/ssh"

var _ ssh.Request

// [ssh.Request]
type Request struct {
	Type      string
	WantReply bool
	Payload   []byte
}

// [ssh.Request.Reply]
func (r *Request) Reply(ok bool, payload []byte) error
