package netlink

import "net"

type LinkAttrs struct {
	*net.Interface
	// ...
}
