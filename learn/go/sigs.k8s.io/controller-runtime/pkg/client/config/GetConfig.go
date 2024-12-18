package config

import "k8s.io/client-go/rest"

func GetConfig() (*rest.Config, error)
