package x509

type CertPool struct{}

func NewCertPool() *CertPool
func SystemCertPool() (*CertPool, error)
func (s *CertPool) AppendCertsFromPEM(pemCerts []byte) (ok bool)
