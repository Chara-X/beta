package netlink

import "net"

type Addr struct {
	*net.IPNet
	// ...
}
