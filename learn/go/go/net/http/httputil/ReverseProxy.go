package httputil

import (
	"log"
	"net/http"
)

type ReverseProxy struct {
	Director       func(*http.Request)
	Transport      http.RoundTripper
	ModifyResponse func(*http.Response) error
	ErrorHandler   func(http.ResponseWriter, *http.Request, error)
	ErrorLog       *log.Logger
}

func (proxy *ReverseProxy) ServeHTTP(w http.ResponseWriter, r *http.Request)
