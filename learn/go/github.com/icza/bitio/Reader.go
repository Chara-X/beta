package bitio

import "io"

type Reader struct{ io.Reader }

func (r *Reader) ReadBits(n uint8) (u uint64, err error)
func (r *Reader) ReadBool() (b bool, err error)
func (r *Reader) Align() (skipped uint8)
