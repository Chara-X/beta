package client

import (
	"context"

	clientv3 "go.etcd.io/etcd/client/v3"
)

type KV interface {
	Put(ctx context.Context, key, val string, opts ...OpOption) (*clientv3.PutResponse, error)
	Get(ctx context.Context, key string, opts ...OpOption) (*clientv3.GetResponse, error)
	Delete(ctx context.Context, key string, opts ...OpOption) (*clientv3.DeleteResponse, error)
	Txn(ctx context.Context) clientv3.Txn
	// Compact(ctx context.Context, rev int64, opts ...CompactOption) (*clientv3.CompactResponse, error)
}
