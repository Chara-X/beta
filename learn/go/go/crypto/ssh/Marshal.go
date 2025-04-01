package ssh

import "golang.org/x/crypto/ssh"

var _ = ssh.Marshal

// [ssh.Marshal]
func Marshal(msg any) []byte
