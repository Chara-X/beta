package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.Playwright]
type Playwright struct {
	Chromium BrowserType
	Firefox  BrowserType
	Webkit   BrowserType
}

// [playwright.Run]
func Run(options ...*playwright.RunOptions)

// [playwright.Playwright.Stop]
func (p *Playwright) Stop() error
