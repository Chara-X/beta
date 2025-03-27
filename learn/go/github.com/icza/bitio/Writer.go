package bitio

import "io"

type Writer struct{ io.Writer }

func (w *Writer) WriteBits(r uint64, n uint8) (err error)
func (w *Writer) WriteBool(b bool) (err error)
func (w *Writer) Close() (err error)
