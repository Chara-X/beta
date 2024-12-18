package ecdh

type PublicKey struct{}

func NewPublicKey(key []byte) (*PublicKey, error)
func (k *PublicKey) Bytes() []byte
