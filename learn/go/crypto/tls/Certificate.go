package tls

import (
	"crypto"
)

type Certificate struct {
	PrivateKey crypto.PrivateKey
	// ...
}

func LoadX509KeyPair(certFile, keyFile string) (Certificate, error)
