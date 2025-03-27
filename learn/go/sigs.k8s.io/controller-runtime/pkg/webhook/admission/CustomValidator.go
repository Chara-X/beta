package admission

import (
	"context"

	"k8s.io/apimachinery/pkg/runtime"
)

type CustomValidator interface {
	ValidateCreate(ctx context.Context, obj runtime.Object) (warnings []string, err error)
	ValidateUpdate(ctx context.Context, oldObj, newObj runtime.Object) (warnings []string, err error)
	ValidateDelete(ctx context.Context, obj runtime.Object) (warnings []string, err error)
}
