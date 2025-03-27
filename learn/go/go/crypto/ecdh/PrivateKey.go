package ecdh

import "io"

type PrivateKey struct{}

func NewPrivateKey(key []byte) (*PrivateKey, error)
func GenerateKey(rand io.Reader) (*PrivateKey, error)
func (k *PrivateKey) Bytes() []byte
func (k *PrivateKey) PublicKey() *PublicKey
func (k *PrivateKey) ECDH(remote *PublicKey) ([]byte, error)
