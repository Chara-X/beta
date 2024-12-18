package crypto

type PublicKey interface {
	Equal(x PublicKey) bool
}
