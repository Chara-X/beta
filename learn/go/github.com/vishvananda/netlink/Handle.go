package netlink

import "github.com/vishvananda/netns"

type Handle struct{}

func NewHandle() (*Handle, error)
func NewHandleAt(ns netns.NsHandle) (*Handle, error)
func (h *Handle) LinkAdd(link Link) error
func (h *Handle) LinkSetNsFd(link Link, fd int) error
func (h *Handle) LinkSetMaster(link Link, master Link) error
func (h *Handle) LinkSetUp(link Link) error
func (h *Handle) AddrAdd(link Link, addr *Addr) error

// ...
