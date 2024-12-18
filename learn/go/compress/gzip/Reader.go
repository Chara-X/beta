package gzip

import (
	"compress/gzip"
)

type Reader struct{ z *gzip.Reader }

func (r *Reader) Read(p []byte) (n int, err error) {
	if Reference {
		return r.z.Read(p)
	}
	panic("not implemented")
}
func (r *Reader) Close() error {
	if Reference {
		return r.z.Close()
	}
	panic("not implemented")
}
