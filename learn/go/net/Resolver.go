package net

import (
	"context"
)

type Resolver struct{}

func (r *Resolver) LookupNS(ctx context.Context, name string) ([]string, error)
func (r *Resolver) LookupAddr(ctx context.Context, name string) ([]string, error)
func (r *Resolver) LookupCNAME(ctx context.Context, name string) (string, error)
func (r *Resolver) LookupTXT(ctx context.Context, name string) ([]string, error)

// ...

/*
LookupAddr: LookupHost
/etc/resolv.conf
*/
