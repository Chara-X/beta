package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.BrowserType]
type BrowserType interface {
	Name() string
	ExecutablePath() string
	Launch(options ...playwright.BrowserTypeLaunchOptions) (Browser, error)
	Connect(wsEndpoint string, options ...playwright.BrowserTypeConnectOptions) (Browser, error)
}
