package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.Page]
type Page interface {
	EventEmitter
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
	Route(url interface{}, handler func(playwright.Route), times ...int) error
	Unroute(url interface{}, handler ...func(playwright.Route)) error
	Close(options ...playwright.PageCloseOptions) error
	WaitForURL(url interface{}, options ...playwright.PageWaitForURLOptions) error
	WaitForFunction(expression string, arg interface{}, options ...playwright.PageWaitForFunctionOptions) (playwright.JSHandle, error)
	WaitForEvent(event string, options ...playwright.PageWaitForEventOptions) (interface{}, error)
}
