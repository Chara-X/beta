package net

import "net"

type Listener interface {
	Addr() net.Addr
	Accept() (Conn, error)
	Close() error
}

func Listen(network, address string) (Listener, error)
