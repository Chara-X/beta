package playwright

import "github.com/playwright-community/playwright-go"

type BrowserType interface {
	Name() string
	ExecutablePath() string
	Launch(options ...playwright.BrowserTypeLaunchOptions) (Browser, error)
	Connect(wsEndpoint string, options ...playwright.BrowserTypeConnectOptions) (Browser, error)
}
type browserType struct{ b playwright.BrowserType }

func (b *browserType) Name() string           { return b.b.Name() }
func (b *browserType) ExecutablePath() string { return b.b.ExecutablePath() }
func (b *browserType) Launch(options ...playwright.BrowserTypeLaunchOptions) (Browser, error) {
	var br, err = b.b.Launch(options...)
	return &browser{b: br}, err
}
func (b *browserType) Connect(wsEndpoint string, options ...playwright.BrowserTypeConnectOptions) (Browser, error) {
	var br, err = b.b.Connect(wsEndpoint, options...)
	return &browser{b: br}, err
}
