package client

import (
	"context"

	clientv3 "go.etcd.io/etcd/client/v3"
)

type Lease interface {
	Grant(ctx context.Context, ttl int64) (*clientv3.LeaseGrantResponse, error)
	Revoke(ctx context.Context, id int64) (*clientv3.LeaseRevokeResponse, error)
	TimeToLive(ctx context.Context, id int64, opts ...clientv3.LeaseOption) (*clientv3.LeaseTimeToLiveResponse, error)
	Leases(ctx context.Context) (*clientv3.LeaseLeasesResponse, error)
	// KeepAlive(ctx context.Context, id clientv3.LeaseID) (<-chan *clientv3.LeaseKeepAliveResponse, error)
	// KeepAliveOnce(ctx context.Context, id clientv3.LeaseID) (*clientv3.LeaseKeepAliveResponse, error)
}
