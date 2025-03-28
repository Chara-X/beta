package grpc

import (
	"context"

	"google.golang.org/grpc/metadata"
)

type ServerStream interface {
	Context() context.Context
	SetHeader(metadata.MD) error
	SetTrailer(metadata.MD)
	RecvMsg(msg any) error
	SendMsg(msg any) error
}
