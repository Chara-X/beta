package crypto

type PrivateKey interface {
	Public() PublicKey
	Equal(x PrivateKey) bool
}
