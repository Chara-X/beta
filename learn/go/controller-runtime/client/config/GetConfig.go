package config

import (
	"k8s.io/client-go/rest"
	"sigs.k8s.io/controller-runtime/pkg/client/config"
)

var _ = config.GetConfig

// [config.GetConfig]
func GetConfig() (*rest.Config, error)
