package client

import (
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/client-go/rest"
	"sigs.k8s.io/controller-runtime/pkg/client"
)

type Client interface {
	client.Reader
	client.Writer
	Scheme() *runtime.Scheme
}

func New(cfg *rest.Config, opts struct {
	Scheme *runtime.Scheme
	Cache  *struct{ Reader client.Reader }
}) (Client, error)

// Reader.Watch(ctx context.Context, obj ObjectList, opts ...ListOption) (watch.Interface, error)
