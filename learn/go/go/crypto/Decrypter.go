package crypto

import "io"

type Decrypter interface {
	Public() PublicKey
	Decrypt(rand io.Reader, data []byte) (plaintext []byte, err error)
}
