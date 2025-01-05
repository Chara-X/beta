package playwright

import "github.com/playwright-community/playwright-go"

type Locator interface {
	All() ([]playwright.Locator, error)
	Locator(selectorOrLocator interface{}, options ...playwright.LocatorLocatorOptions) Locator
	Evaluate(expression string, arg interface{}, options ...playwright.LocatorEvaluateOptions) (interface{}, error)
	EvaluateHandle(expression string, arg interface{}, options ...playwright.LocatorEvaluateHandleOptions) (playwright.JSHandle, error)
	Screenshot(options ...playwright.LocatorScreenshotOptions) ([]byte, error)
	WaitFor(options ...playwright.LocatorWaitForOptions) error
	Err() error
}
type locator struct{ l playwright.Locator }

func (l *locator) All() ([]playwright.Locator, error) {
	return l.l.All()
}
func (l *locator) Locator(selectorOrLocator interface{}, options ...playwright.LocatorLocatorOptions) Locator {
	return &locator{l: l.l.Locator(selectorOrLocator, options...)}
}
func (l *locator) Evaluate(expression string, arg interface{}, options ...playwright.LocatorEvaluateOptions) (interface{}, error) {
	return l.l.Evaluate(expression, arg, options...)
}
func (l *locator) EvaluateHandle(expression string, arg interface{}, options ...playwright.LocatorEvaluateHandleOptions) (playwright.JSHandle, error) {
	return l.l.EvaluateHandle(expression, arg, options...)
}
func (l *locator) Screenshot(options ...playwright.LocatorScreenshotOptions) ([]byte, error) {
	return l.l.Screenshot(options...)
}
func (l *locator) WaitFor(options ...playwright.LocatorWaitForOptions) error {
	return l.l.WaitFor(options...)
}
func (l *locator) Err() error {
	return l.l.Err()
}
