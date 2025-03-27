package tls

import "crypto/tls"

const (
	NoClientCert tls.ClientAuthType = iota
	RequestClientCert
	RequireAnyClientCert
	VerifyClientCertIfGiven
	RequireAndVerifyClientCert
)
