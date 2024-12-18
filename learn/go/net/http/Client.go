package http

import (
	"net/http"
)

type Client struct {
	Transport     *Transport
	CheckRedirect func(req *http.Request, via []*http.Request) error
	Jar           http.CookieJar
}

func (c *Client) Do(req *http.Request) (*http.Response, error)
