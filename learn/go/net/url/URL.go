package url

import "net/url"

type URL struct {
	Scheme   string
	Opaque   string
	User     *url.Userinfo
	Host     string
	Path     string
	Query    url.Values
	Fragment string
}

func Parse(rawURL string) (*URL, error)
func (url *URL) String() string

/*
Query url.Values: Query() url.Values + RawQuery string
u.IsAbs(): u.Scheme != ""
*/
