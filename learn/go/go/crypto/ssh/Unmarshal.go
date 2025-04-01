package ssh

import "golang.org/x/crypto/ssh"

var _ = ssh.Unmarshal

// [ssh.Unmarshal]
func Unmarshal(data []byte, out any) error
