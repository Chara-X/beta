package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.Browser]
type Browser interface {
	EventEmitter
	Version() string
	NewPage(options ...playwright.BrowserNewPageOptions) (Page, error)
	Close(options ...playwright.BrowserCloseOptions) error
}
