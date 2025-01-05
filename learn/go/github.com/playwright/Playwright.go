package playwright

import "github.com/playwright-community/playwright-go"

type Playwright struct {
	p        *playwright.Playwright
	Chromium BrowserType
	Firefox  BrowserType
	Webkit   BrowserType
}

func Run(options ...*playwright.RunOptions) (*Playwright, error) {
	var p, err = playwright.Run(options...)
	return &Playwright{p: p, Chromium: &browserType{b: p.Chromium}}, err
}
func (p *Playwright) Stop() error { return p.p.Stop() }
