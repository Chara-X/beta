package playwright

import "github.com/playwright-community/playwright-go"

var _ playwright.JSHandle

// [playwright.JSHandle]
type JSHandle interface {
	GetProperties() (map[string]JSHandle, error)
	JSONValue() (interface{}, error)
	Evaluate(expression string, arg ...interface{}) (interface{}, error)
	EvaluateHandle(expression string, arg ...interface{}) (JSHandle, error)
	Dispose() error
}
