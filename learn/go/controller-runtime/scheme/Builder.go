package scheme

import (
	"k8s.io/apimachinery/pkg/runtime"
	"k8s.io/apimachinery/pkg/runtime/schema"
	"sigs.k8s.io/controller-runtime/pkg/scheme"
)

var _ scheme.Builder

// [scheme.Builder]
type Builder struct {
	runtime.SchemeBuilder
	GroupVersion schema.GroupVersion
}

// [scheme.Builder.Register]
func (bld *Builder) Register(object ...runtime.Object) *Builder

// [scheme.Builder.AddToScheme]
func (bld *Builder) AddToScheme(s *runtime.Scheme) error
