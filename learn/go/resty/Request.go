package resty

import (
	"net/http"
	"net/url"

	"resty.dev/v3"
)

// [resty.Request]
type Request struct {
	Method        string
	URL           string
	PathParams    map[string]string
	QueryParams   url.Values
	Header        http.Header
	Cookies       []*http.Cookie
	AuthScheme    string
	AuthToken     string
	FormData      url.Values
	RetryStrategy resty.RetryStrategyFunc
}

// [resty.Request.SetPathParams]
func (r *Request) SetPathParams(params map[string]string) *Request

// [resty.Request.SetQueryParams]
func (r *Request) SetQueryParams(params map[string]string) *Request

// [resty.Request.SetHeaders]
func (r *Request) SetHeaders(headers map[string]string) *Request

// [resty.Request.SetContentType]
func (r *Request) SetContentType(ct string) *Request

// [resty.Request.SetBody]
func (r *Request) SetBody(body any) *Request

// [resty.Request.SetResult]
func (r *Request) SetResult(v any) *Request

// [resty.Request.SetError]
func (r *Request) SetError(v any) *Request

// [resty.Request.Execute]
func (r *Request) Execute(method, url string) (res *Response, err error)
