package http

import (
	"crypto/tls"
	"net"
	"net/http"
	"net/url"
)

type Transport struct {
	Proxy           func(*http.Request) (*url.URL, error)
	Dial            func(network, addr string) (net.Conn, error)
	TLSClientConfig *tls.Config
	// ...
}

func (t *Transport) RoundTrip(req *http.Request) (*http.Response, error)
