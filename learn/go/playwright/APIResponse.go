package playwright

import "github.com/playwright-community/playwright-go"

var _ playwright.APIResponse

// [playwright.APIResponse]
type APIResponse interface {
	Status() int
	StatusText() string
	Ok() bool
	URL() string
	Headers() map[string]string
	Body() ([]byte, error)
	Text() (string, error)
	JSON(v interface{}) error
	Dispose() error
}
