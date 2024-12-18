package http

import (
	"net/http"
	"net/url"
)

func ProxyFromEnvironment(req *http.Request) (*url.URL, error)
