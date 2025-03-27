package client

import (
	"context"

	clientv3 "go.etcd.io/etcd/client/v3"
)

type Maintenance interface {
	Status(ctx context.Context, endpoint string) (*clientv3.StatusResponse, error)
	// AlarmList(ctx context.Context) (*clientv3.AlarmResponse, error)
	// AlarmDisarm(ctx context.Context, m *clientv3.AlarmMember) (*clientv3.AlarmResponse, error)
	// HashKV(ctx context.Context, endpoint string, rev int64) (*clientv3.HashKVResponse, error)
	// Snapshot(ctx context.Context) (io.ReadCloser, error)
	// MoveLeader(ctx context.Context, transfereeID uint64) (*clientv3.MoveLeaderResponse, error)
}
