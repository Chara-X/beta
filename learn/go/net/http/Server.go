package http

import (
	"context"
	"crypto/tls"
)

type Server struct {
	Addr      string
	Handler   Handler
	TLSConfig *tls.Config
	// ...
}

func (srv *Server) RegisterOnShutdown(f func())
func (srv *Server) ListenAndServeTLS(certFile, keyFile string) error
func (srv *Server) ListenAndServe() error
func (srv *Server) Shutdown(ctx context.Context) error
