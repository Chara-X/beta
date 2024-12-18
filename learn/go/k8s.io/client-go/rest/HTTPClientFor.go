package rest

import (
	"net/http"

	"k8s.io/client-go/rest"
)

func HTTPClientFor(config *rest.Config) (*http.Client, error)
