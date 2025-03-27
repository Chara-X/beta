package net

import (
	"net"
)

type Conn interface {
	LocalAddr() net.Addr
	RemoteAddr() net.Addr
	Read(b []byte) (n int, err error)
	Write(b []byte) (n int, err error)
	Close() error
	// ...
}

func Dial(network, address string) (Conn, error)
