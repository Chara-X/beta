package hash

import "io"

type Hash interface {
	io.Writer
	Sum(b []byte) []byte
	Reset()
}
