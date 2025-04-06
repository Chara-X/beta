package graphql

import (
	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/gqlerrors"
)

// [graphql.Result]
type Result struct {
	Data   any                        `json:"data"`
	Errors []gqlerrors.FormattedError `json:"errors,omitempty"`
}

// [graphql.Do]
func Do(p graphql.Params) *Result
