package testing

import "testing"

// [testing.T]
type T struct{ t *testing.T }

func (c *T) Name() string    { return c.t.Name() }
func (c *T) TempDir() string { return c.t.TempDir() }

// [testing.T.Setenv]
func (t *T) Setenv(key, value string)

// [testing.T.Run]
func (t *T) Run(name string, f func(t *T)) bool

// [testing.T.Parallel]
func (t *T) Parallel()
func (c *T) Log(args ...any) { c.t.Log(args...) }
func (c *T) Fatal()          { c.t.Fatal() }
