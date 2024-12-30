package playwright

import "github.com/playwright-community/playwright-go"

type Browser interface {
	playwright.EventEmitter
	Version() string
	BrowserType() playwright.BrowserType
	Contexts() []playwright.BrowserContext
	NewPage(options ...playwright.BrowserNewPageOptions) (playwright.Page, error)
	NewBrowserCDPSession() (playwright.CDPSession, error)
	NewContext(options ...playwright.BrowserNewContextOptions) (playwright.BrowserContext, error)
	Close(options ...playwright.BrowserCloseOptions) error
	StartTracing(options ...playwright.BrowserStartTracingOptions) error
	StopTracing() ([]byte, error)
	OnDisconnected(fn func(Browser))
}
