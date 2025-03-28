package client

import (
	"context"

	clientv3 "go.etcd.io/etcd/client/v3"
)

type Watcher interface {
	Watch(ctx context.Context, key string, opts ...OpOption) <-chan clientv3.WatchResponse
}
