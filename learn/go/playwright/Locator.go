package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.Locator]
type Locator interface {
	All() ([]playwright.Locator, error)
	And(locator Locator) Locator
	Or(locator Locator) Locator
	Locator(selectorOrLocator interface{}, options ...playwright.LocatorLocatorOptions) Locator
	GetByAltText(text interface{}, options ...playwright.LocatorGetByAltTextOptions) Locator
	GetByLabel(text interface{}, options ...playwright.LocatorGetByLabelOptions) Locator
	GetByPlaceholder(text interface{}, options ...playwright.LocatorGetByPlaceholderOptions) Locator
	GetByText(text interface{}, options ...playwright.LocatorGetByTextOptions) Locator
	GetByTitle(text interface{}, options ...playwright.LocatorGetByTitleOptions) Locator
	GetByRole(role playwright.AriaRole, options ...playwright.LocatorGetByRoleOptions) Locator
	Filter(options ...playwright.LocatorFilterOptions) Locator
	Evaluate(expression string, arg interface{}, options ...playwright.LocatorEvaluateOptions) (interface{}, error)
	EvaluateHandle(expression string, arg interface{}, options ...playwright.LocatorEvaluateHandleOptions) (playwright.JSHandle, error)
	Screenshot(options ...playwright.LocatorScreenshotOptions) ([]byte, error)
	AriaSnapshot(options ...playwright.LocatorAriaSnapshotOptions) (string, error)
	WaitFor(options ...playwright.LocatorWaitForOptions) error
	Err() error
}
