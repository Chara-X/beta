package client

import (
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/types"
	"k8s.io/client-go/rest"
	"sigs.k8s.io/controller-runtime/pkg/client"
)

// [client.Client]
type Client interface {
	client.Reader
	client.Writer
	client.StatusClient
	Scheme() *runtime.Scheme
}

// [client.New]
func New(config *rest.Config, options client.Options) (Client, error)

// [client.NewNamespacedClient]
func NewNamespacedClient(c Client, ns string) Client

// [client.Patch]
type Patch interface {
	Type() types.PatchType
	Data(obj client.Object) ([]byte, error)
}

// [client.MergeFrom]
func MergeFrom(obj client.Object) Patch

// [client.StrategicMergeFrom]
func StrategicMergeFrom(obj client.Object, opts ...client.MergeFromOption) Patch
