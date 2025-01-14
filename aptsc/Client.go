package aptsc

import (
	"bytes"
	"encoding/json"
	"net/http"

	"github.com/Chara-X/aptsc/msg"
)

type Client struct {
	c    *http.Client
	addr string
}

func New(c *http.Client, addr string) *Client {
	return &Client{c: c, addr: addr}
}
func (c *Client) Create(req interface{}) {
	var path string
	switch req.(type) {
	case *msg.HealthCheckRequest:
		path = "/opapi/wsm/v1/apts/healthcheck"
	default:
		panic("not supported type")
	}
	var buf = new(bytes.Buffer)
	json.NewEncoder(buf).Encode(req)
	var res, err = c.c.Post(c.addr+path, "application/json", buf)
	if err != nil {
		panic(err)
	}
	res.Body.Close()
}
