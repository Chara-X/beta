package gzip

import (
	"compress/gzip"
	"io"
)

type Writer struct{ z *gzip.Writer }

func NewWriter(w io.Writer) *Writer {
	if Reference {
		return &Writer{gzip.NewWriter(w)}
	}
	panic("not implemented")
}
func (w *Writer) Write(p []byte) (int, error) {
	if Reference {
		return w.z.Write(p)
	}
	panic("not implemented")
}
func (w *Writer) Close() error {
	if Reference {
		return w.z.Close()
	}
	panic("not implemented")
}
