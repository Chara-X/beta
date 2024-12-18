package grpc

import (
	"net/http"

	"google.golang.org/grpc"
)

type Server struct{}

func NewServer(opts ...grpc.ServerOption) *Server
func (s *Server) RegisterService(desc *grpc.ServiceDesc, srv any)
func (s *Server) ServeHTTP(w http.ResponseWriter, r *http.Request)
