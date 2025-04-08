package resty

import (
	"resty.dev/v3"
)

var _ resty.Client

// [resty.Client]
type Client struct{}

// [resty.New]
func New() *Client

// [resty.Client.SetBaseURL]
func (c *Client) SetBaseURL(url string) *Client

// [resty.Client.SetPathParams]
func (c *Client) SetPathParams(params map[string]string) *Client

// [resty.Client.SetQueryParams]
func (c *Client) SetQueryParams(params map[string]string) *Client

// [resty.Client.SetHeaders]
func (c *Client) SetHeaders(headers map[string]string) *Client

// [resty.Client.SetAuthToken]
func (c *Client) SetAuthToken(token string) *Client

// [resty.Client.SetBasicAuth]
func (c *Client) SetBasicAuth(username, password string) *Client

// [resty.Client.SetError]
func (c *Client) SetError(v any) *Client

// [resty.Client.R]
func (c *Client) R() *Request
