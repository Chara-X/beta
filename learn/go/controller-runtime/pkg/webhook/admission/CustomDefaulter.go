package admission

import (
	"context"

	"k8s.io/apimachinery/pkg/runtime"
)

type CustomDefaulter interface {
	Default(ctx context.Context, obj runtime.Object) error
}
