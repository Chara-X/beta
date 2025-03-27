package client

type Client struct {
	KV
	Watcher
	Lease
	// Cluster
	// Auth
	// Maintenance
	Username string
	Password string
}

func New(cfg Config) (*Client, error)
func (c *Client) Close() error
