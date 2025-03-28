package admission

import (
	"net/http"

	"k8s.io/apimachinery/pkg/runtime"
	"sigs.k8s.io/controller-runtime/pkg/webhook/admission"
)

type Webhook struct{ Handler admission.Handler }

func WithCustomDefaulter(scheme *runtime.Scheme, objType runtime.Object, defaulter CustomDefaulter) *Webhook
func WithCustomValidator(scheme *runtime.Scheme, objType runtime.Object, validator CustomValidator) *Webhook
func (wh *Webhook) ServeHTTP(w http.ResponseWriter, r *http.Request)
