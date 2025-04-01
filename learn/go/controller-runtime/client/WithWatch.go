package client

import (
	"context"

	"k8s.io/apimachinery/pkg/watch"
	"k8s.io/client-go/rest"
	"sigs.k8s.io/controller-runtime/pkg/client"
)

// [client.WithWatch]
type WithWatch interface {
	Client
	Watch(ctx context.Context, obj client.ObjectList, opts ...client.ListOption) (watch.Interface, error)
}

// [client.NewWithWatch]
func NewWithWatch(config *rest.Config, options client.Options) (WithWatch, error)
