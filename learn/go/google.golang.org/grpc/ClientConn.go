package grpc

import (
	"context"

	"google.golang.org/grpc"
)

type ClientConn struct{}

func NewClient(target string, opts ...grpc.DialOption) (conn *ClientConn, err error)
func (cc *ClientConn) NewStream(ctx context.Context, desc *grpc.StreamDesc, method string, opts ...grpc.CallOption) (ClientStream, error)

// ...
