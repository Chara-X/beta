package playwright

import "github.com/playwright-community/playwright-go"

type Page interface {
	Title() (string, error)
	URL() string
	Request() APIRequestContext
	Clock() playwright.Clock
	Keyboard() playwright.Keyboard
	Mouse() playwright.Mouse
	Video() playwright.Video
	Goto(url string, options ...playwright.PageGotoOptions) (playwright.Response, error)
	GoBack(options ...playwright.PageGoBackOptions) (playwright.Response, error)
	GoForward(options ...playwright.PageGoForwardOptions) (playwright.Response, error)
	Locator(selector string, options ...playwright.PageLocatorOptions) Locator
	Evaluate(expression string, arg ...interface{}) (interface{}, error)
	EvaluateHandle(expression string, arg ...interface{}) (playwright.JSHandle, error)
	Route(url interface{}, handler func(playwright.Route), times ...int) error
	Unroute(url interface{}, handler ...func(playwright.Route)) error
	Screenshot(options ...playwright.PageScreenshotOptions) ([]byte, error)
	Close(options ...playwright.PageCloseOptions) error
	WaitForURL(url interface{}, options ...playwright.PageWaitForURLOptions) error
	WaitForFunction(expression string, arg interface{}, options ...playwright.PageWaitForFunctionOptions) (playwright.JSHandle, error)
	WaitForEvent(event string, options ...playwright.PageWaitForEventOptions) (interface{}, error)
}
type page struct{ p playwright.Page }

func (p *page) Title() (string, error) {
	return p.p.Title()
}
func (p *page) URL() string {
	return p.p.URL()
}
func (p *page) Request() APIRequestContext {
	return &apiRequestContext{r: p.p.Request()}
}
func (p *page) Clock() playwright.Clock {
	return p.p.Clock()
}
func (p *page) Keyboard() playwright.Keyboard {
	return p.p.Keyboard()
}
func (p *page) Mouse() playwright.Mouse {
	return p.p.Mouse()
}
func (p *page) Video() playwright.Video {
	return p.p.Video()
}
func (p *page) Goto(url string, options ...playwright.PageGotoOptions) (playwright.Response, error) {
	return p.p.Goto(url, options...)
}
func (p *page) GoBack(options ...playwright.PageGoBackOptions) (playwright.Response, error) {
	return p.p.GoBack(options...)
}
func (p *page) GoForward(options ...playwright.PageGoForwardOptions) (playwright.Response, error) {
	return p.p.GoForward(options...)
}
func (p *page) Locator(selector string, options ...playwright.PageLocatorOptions) Locator {
	return &locator{l: p.p.Locator(selector, options...)}
}
func (p *page) Evaluate(expression string, arg ...interface{}) (interface{}, error) {
	return p.p.Evaluate(expression, arg...)
}
func (p *page) EvaluateHandle(expression string, arg ...interface{}) (playwright.JSHandle, error) {
	return p.p.EvaluateHandle(expression, arg...)
}
func (p *page) Route(url interface{}, handler func(playwright.Route), times ...int) error {
	return p.p.Route(url, handler, times...)
}
func (p *page) Unroute(url interface{}, handler ...func(playwright.Route)) error {
	return p.p.Unroute(url, handler...)
}
func (p *page) Screenshot(options ...playwright.PageScreenshotOptions) ([]byte, error) {
	return p.p.Screenshot(options...)
}
func (p *page) Close(options ...playwright.PageCloseOptions) error {
	return p.p.Close(options...)
}
func (p *page) WaitForURL(url interface{}, options ...playwright.PageWaitForURLOptions) error {
	return p.p.WaitForURL(url, options...)
}
func (p *page) WaitForFunction(expression string, arg interface{}, options ...playwright.PageWaitForFunctionOptions) (playwright.JSHandle, error) {
	return p.p.WaitForFunction(expression, arg, options...)
}
func (p *page) WaitForEvent(event string, options ...playwright.PageWaitForEventOptions) (interface{}, error) {
	return p.p.WaitForEvent(event, options...)
}
