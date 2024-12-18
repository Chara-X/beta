package http

import "net/http"

type ResponseWriter interface {
	Header() http.Header
	WriteHeader(statusCode int)
	Write([]byte) (int, error)
}
