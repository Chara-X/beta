package http

import (
	"net/http"
	"time"
)

type Handler interface {
	ServeHTTP(w ResponseWriter, r *http.Request)
}

func TimeoutHandler(hd Handler, timeout time.Duration, msg string) Handler
func StripPrefix(prefix string, hd Handler) Handler
