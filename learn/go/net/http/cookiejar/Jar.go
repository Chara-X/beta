package cookiejar

import (
	"net/http"
	"net/url"
)

type Jar struct{}

func New() (*Jar, error)
func (j *Jar) Cookies(u *url.URL) (cookies []*http.Cookie)
func (j *Jar) SetCookies(u *url.URL, cookies []*http.Cookie)
