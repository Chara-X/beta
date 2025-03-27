// [testing]
package testing

import "testing"

var _ testing.M

// [testing.M]
type M struct{}

// [testing.M.Run]
func (m *M) Run() (code int)
