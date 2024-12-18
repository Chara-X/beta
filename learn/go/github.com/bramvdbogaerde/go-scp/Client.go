package scp

import (
	"context"
	"os"

	"golang.org/x/crypto/ssh"
)

type Client struct{ RemoteBinary string }

func NewClientBySSH(ssh *ssh.Client) (Client, error)
func (c *Client) CopyFromFile(ctx context.Context, file os.File, remotePath string, permissions string) error
func (c *Client) CopyFromRemote(ctx context.Context, file *os.File, remotePath string) error
func (c *Client) Close()
