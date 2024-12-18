package grpc

import (
	"google.golang.org/grpc/metadata"
)

type ClientStream interface {
	Header() (metadata.MD, error)
	Trailer() metadata.MD
	RecvMsg(msg any) error
	SendMsg(msg any) error
	CloseSend() error
}
