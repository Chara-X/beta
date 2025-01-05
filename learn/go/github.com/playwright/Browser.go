package playwright

import "github.com/playwright-community/playwright-go"

type Browser interface {
	Version() string
	NewPage(options ...playwright.BrowserNewPageOptions) (Page, error)
	Close(options ...playwright.BrowserCloseOptions) error
}
type browser struct{ b playwright.Browser }

func (b *browser) Version() string { return b.b.Version() }
func (b *browser) NewPage(options ...playwright.BrowserNewPageOptions) (Page, error) {
	var p, err = b.b.NewPage(options...)
	return &page{p: p}, err
}
func (b *browser) Close(options ...playwright.BrowserCloseOptions) error {
	return b.b.Close(options...)
}
