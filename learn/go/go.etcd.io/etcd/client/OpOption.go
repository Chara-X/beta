package client

import clientv3 "go.etcd.io/etcd/client/v3"

type OpOption func(*clientv3.Op)

func WithRange(endKey string) OpOption
func WithFromKey() OpOption
func WithPrefix() OpOption
