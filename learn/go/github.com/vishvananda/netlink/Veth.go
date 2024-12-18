package netlink

import "net"

type Veth struct {
	LinkAttrs
	PeerName         string
	PeerHardwareAddr net.HardwareAddr
}

func (veth *Veth) Attrs() *LinkAttrs
func (veth *Veth) Type() string
