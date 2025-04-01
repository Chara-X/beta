package ssh

import "golang.org/x/crypto/ssh"

var _ ssh.AuthMethod

// [ssh.AuthMethod]
type AuthMethod any

// [ssh.Password]
func Password(secret string) AuthMethod
