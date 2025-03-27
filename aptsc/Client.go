package aptsc

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"io/fs"
	"mime/multipart"
	"net/http"
	"time"
)

type Client struct {
	c   *http.Client
	srv string
}

func New(c *http.Client, srv string) *Client { return &Client{c: c, srv: srv} }
func (c *Client) PostAsJson(path string, body any) []byte {
	var b, err = json.Marshal(body)
	if err != nil {
		panic(err)
	}
	return c.Post(path, b)
}
func (c *Client) GetFromJson(path string, body any) {
	if err := json.Unmarshal(c.Get(path), &body); err != nil {
		panic(err)
	}
}
func (c *Client) Post(path string, body []byte) []byte {
	return c.Do(http.MethodPost, path, bytes.NewReader(body))
}
func (c *Client) Delete(path string) []byte {
	return c.Do(http.MethodDelete, path, nil)
}
func (c *Client) Get(path string) []byte {
	return c.Do(http.MethodGet, path, nil)
}
func (c *Client) Do(method, path string, body io.Reader) []byte {
	var req, _ = http.NewRequest(method, c.srv+path, body)
	var res, err = c.c.Do(req)
	if err != nil {
		panic(err)
	}
	defer res.Body.Close()
	var resBody, _ = io.ReadAll(res.Body)
	if res.StatusCode >= http.StatusBadRequest {
		panic(errors.New(path + string(resBody)))
	}
	return resBody
}
func (c *Client) Polling(timeout time.Duration, f func() bool) {
	for i := 0; i < int(timeout/(5*time.Second)); i++ {
		if !f() {
			return
		}
		time.Sleep(5 * time.Second)
	}
	panic(errors.New("polling timeout"))
}
func (c *Client) PostFile(path string, fs fs.FS, name string) {
	f, err := fs.Open(name)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	var body = &bytes.Buffer{}
	var w = multipart.NewWriter(body)
	var stat, _ = f.Stat()
	var part, _ = w.CreateFormFile("file", stat.Name())
	if _, err := io.Copy(part, f); err != nil {
		panic(err)
	}
	w.Close()
	res, err := c.c.Post(c.srv+path, w.FormDataContentType(), body)
	if err != nil {
		panic(err)
	}
	defer res.Body.Close()
	if res.StatusCode >= http.StatusBadRequest {
		var body, _ = io.ReadAll(res.Body)
		panic(string(body))
	}
}
