package net

import "net"

type Interface struct {
	Index        int
	Name         string
	HardwareAddr net.HardwareAddr
	// ...
}

func Interfaces() ([]Interface, error)
func InterfaceByName(name string) (*Interface, error)
func (ifi *Interface) Addrs() ([]net.Addr, error)
