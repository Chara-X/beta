package crypto

import (
	"io"
)

type Signer interface {
	Public() PublicKey
	Sign(rand io.Reader, data []byte) (signature []byte, err error)
}
