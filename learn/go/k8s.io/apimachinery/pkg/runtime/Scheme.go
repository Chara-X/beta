package runtime

import (
	"reflect"

	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/runtime/schema"
)

type Scheme struct{}

func NewScheme() *Scheme
func (s *Scheme) AddKnownTypes(gv schema.GroupVersion, types ...runtime.Object)
func (s *Scheme) AllKnownTypes() map[schema.GroupVersionKind]reflect.Type

// ...
