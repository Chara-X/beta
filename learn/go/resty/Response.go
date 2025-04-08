package resty

import (
	"io"
	"net/http"

	"resty.dev/v3"
)

var _ resty.Response

// [resty.Response]
type Response struct {
	Request *Request
	Body    io.ReadCloser
	Err     error
}

// [resty.Response.StatusCode]
func (r *Response) StatusCode() int

// [resty.Response.Header]
func (r *Response) Header() http.Header

// [resty.Response.Cookies]
func (r *Response) Cookies() []*http.Cookie

// [resty.Response.Size]
func (r *Response) Size() int64
