package client

import (
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/client-go/rest"
	"sigs.k8s.io/controller-runtime/pkg/client"
)

// [client.Client]
type Client interface {
	client.Reader
	client.Writer
	Scheme() *runtime.Scheme
}

// [client.New]
func New(config *rest.Config, options client.Options) (Client, error)

// [client.NewNamespacedClient]
func NewNamespacedClient(c Client, ns string) Client
