package playwright

import "github.com/playwright-community/playwright-go"

type Page interface {
	playwright.EventEmitter
	Title() (string, error)
	URL() string
	Goto(url string, options ...playwright.PageGotoOptions) (playwright.Response, error)
	Locator(selector string, options ...playwright.PageLocatorOptions) playwright.Locator
	GetByAltText(text interface{}, options ...playwright.PageGetByAltTextOptions) playwright.Locator
	GetByLabel(text interface{}, options ...playwright.PageGetByLabelOptions) playwright.Locator
	GetByPlaceholder(text interface{}, options ...playwright.PageGetByPlaceholderOptions) playwright.Locator
	GetByRole(role playwright.AriaRole, options ...playwright.PageGetByRoleOptions) playwright.Locator
	GetByTestId(testId interface{}) playwright.Locator
	GetByText(text interface{}, options ...playwright.PageGetByTextOptions) playwright.Locator
	GetByTitle(text interface{}, options ...playwright.PageGetByTitleOptions) playwright.Locator
	Clock() playwright.Clock
	AddInitScript(script playwright.Script) error
	AddScriptTag(options playwright.PageAddScriptTagOptions) (playwright.ElementHandle, error)
	AddStyleTag(options playwright.PageAddStyleTagOptions) (playwright.ElementHandle, error)
	BringToFront() error
	Close(options ...playwright.PageCloseOptions) error
	Content() (string, error)
	Context() playwright.BrowserContext
	DragAndDrop(source string, target string, options ...playwright.PageDragAndDropOptions) error
	EmulateMedia(options ...playwright.PageEmulateMediaOptions) error
	Evaluate(expression string, arg ...interface{}) (interface{}, error)
	EvaluateHandle(expression string, arg ...interface{}) (playwright.JSHandle, error)
	ExposeBinding(name string, binding playwright.BindingCallFunction, handle ...bool) error
	ExposeFunction(name string, binding playwright.ExposedFunction) error
	Frame(options ...playwright.PageFrameOptions) playwright.Frame
	FrameLocator(selector string) playwright.FrameLocator
	Frames() []playwright.Frame
	GoBack(options ...playwright.PageGoBackOptions) (playwright.Response, error)
	GoForward(options ...playwright.PageGoForwardOptions) (playwright.Response, error)
	RequestGC() error
	IsClosed() bool
	Keyboard() playwright.Keyboard
	MainFrame() playwright.Frame
	Mouse() playwright.Mouse
	Opener() (Page, error)
	Pause() error
	PDF(options ...playwright.PagePdfOptions) ([]byte, error)
	AddLocatorHandler(locator playwright.Locator, handler func(playwright.Locator), options ...playwright.PageAddLocatorHandlerOptions) error
	RemoveLocatorHandler(locator playwright.Locator) error
	Reload(options ...playwright.PageReloadOptions) (playwright.Response, error)
	Request() playwright.APIRequestContext
	Route(url interface{}, handler func(playwright.Route), times ...int) error
	RouteFromHAR(har string, options ...playwright.PageRouteFromHAROptions) error
	RouteWebSocket(url interface{}, handler func(playwright.WebSocketRoute)) error
	Screenshot(options ...playwright.PageScreenshotOptions) ([]byte, error)
	SetContent(html string, options ...playwright.PageSetContentOptions) error
	SetDefaultNavigationTimeout(timeout float64)
	SetDefaultTimeout(timeout float64)
	SetExtraHTTPHeaders(headers map[string]string) error
	SetViewportSize(width int, height int) error
	Touchscreen() playwright.Touchscreen
	UnrouteAll(options ...playwright.PageUnrouteAllOptions) error
	Unroute(url interface{}, handler ...func(playwright.Route)) error
	Video() playwright.Video
	ViewportSize() *playwright.Size
	ExpectConsoleMessage(cb func() error, options ...playwright.PageExpectConsoleMessageOptions) (playwright.ConsoleMessage, error)
	ExpectDownload(cb func() error, options ...playwright.PageExpectDownloadOptions) (playwright.Download, error)
	ExpectEvent(event string, cb func() error, options ...playwright.PageExpectEventOptions) (interface{}, error)
	ExpectFileChooser(cb func() error, options ...playwright.PageExpectFileChooserOptions) (playwright.FileChooser, error)
	ExpectPopup(cb func() error, options ...playwright.PageExpectPopupOptions) (Page, error)
	ExpectRequest(urlOrPredicate interface{}, cb func() error, options ...playwright.PageExpectRequestOptions) (playwright.Request, error)
	ExpectRequestFinished(cb func() error, options ...playwright.PageExpectRequestFinishedOptions) (playwright.Request, error)
	ExpectResponse(urlOrPredicate interface{}, cb func() error, options ...playwright.PageExpectResponseOptions) (playwright.Response, error)
	ExpectWebSocket(cb func() error, options ...playwright.PageExpectWebSocketOptions) (playwright.WebSocket, error)
	ExpectWorker(cb func() error, options ...playwright.PageExpectWorkerOptions) (playwright.Worker, error)
	WaitForFunction(expression string, arg interface{}, options ...playwright.PageWaitForFunctionOptions) (playwright.JSHandle, error)
	WaitForLoadState(options ...playwright.PageWaitForLoadStateOptions) error
	WaitForURL(url interface{}, options ...playwright.PageWaitForURLOptions) error
	WaitForEvent(event string, options ...playwright.PageWaitForEventOptions) (interface{}, error)
	Workers() []playwright.Worker
	// OnClose(fn func(Page))
	// OnConsole(fn func(playwright.ConsoleMessage))
	// OnCrash(fn func(Page))
	// OnDialog(fn func(playwright.Dialog))
	// OnDOMContentLoaded(fn func(Page))
	// OnDownload(fn func(playwright.Download))
	// OnFileChooser(fn func(playwright.FileChooser))
	// OnFrameAttached(fn func(playwright.Frame))
	// OnFrameDetached(fn func(playwright.Frame))
	// OnFrameNavigated(fn func(playwright.Frame))
	// OnLoad(fn func(Page))
	// OnPageError(fn func(error))
	// OnPopup(fn func(Page))
	// OnRequest(fn func(playwright.Request))
	// OnRequestFailed(fn func(playwright.Request))
	// OnRequestFinished(fn func(playwright.Request))
	// OnResponse(fn func(playwright.Response))
	// OnWebSocket(fn func(playwright.WebSocket))
	// OnWorker(fn func(playwright.Worker))
}
