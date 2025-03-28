package netlink

type Link interface {
	Attrs() *LinkAttrs
	Type() string
}
