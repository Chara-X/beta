package http

import "net/http"

type ServeMux struct{}

func NewServeMux() *ServeMux
func (mux *ServeMux) Handle(pattern string, handler Handler)
func (mux *ServeMux) ServeHTTP(w ResponseWriter, r *http.Request)
