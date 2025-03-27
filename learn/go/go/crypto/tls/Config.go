package tls

import (
	"crypto/tls"
	"crypto/x509"
)

type Config struct {
	RootCAs      *x509.CertPool
	Certificates []Certificate
	ClientAuth   tls.ClientAuthType
	// ...
}
