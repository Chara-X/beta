package playwright

import (
	"github.com/playwright-community/playwright-go"
)

type Playwright struct {
	p         *playwright.Playwright
	Selectors playwright.Selectors
	Chromium  playwright.BrowserType
	Firefox   playwright.BrowserType
	WebKit    playwright.BrowserType
	Request   playwright.APIRequest
	Devices   map[string]*playwright.DeviceDescriptor
}

func Run(options ...*playwright.RunOptions) (*Playwright, error) {
	var p, err = playwright.Run(options...)
	return &Playwright{p: p}, err
}
