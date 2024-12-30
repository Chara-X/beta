package playwright

import "github.com/playwright-community/playwright-go"

type Locator interface {
	Page() (playwright.Page, error)
	All() ([]playwright.Locator, error)
	And(locator playwright.Locator) playwright.Locator
	Or(locator playwright.Locator) playwright.Locator
	Filter(options ...playwright.LocatorFilterOptions) playwright.Locator
	Locator(selectorOrLocator interface{}, options ...playwright.LocatorLocatorOptions) playwright.Locator
	GetByRole(role playwright.AriaRole, options ...playwright.LocatorGetByRoleOptions) playwright.Locator
	GetByTitle(text interface{}, options ...playwright.LocatorGetByTitleOptions) playwright.Locator
	GetByText(text interface{}, options ...playwright.LocatorGetByTextOptions) playwright.Locator
	GetByAltText(text interface{}, options ...playwright.LocatorGetByAltTextOptions) playwright.Locator
	GetByLabel(text interface{}, options ...playwright.LocatorGetByLabelOptions) playwright.Locator
	GetByPlaceholder(text interface{}, options ...playwright.LocatorGetByPlaceholderOptions) playwright.Locator
	ContentFrame() playwright.FrameLocator
	FrameLocator(selector string) playwright.FrameLocator
	GetAttribute(name string, options ...playwright.LocatorGetAttributeOptions) (string, error)
	TextContent(options ...playwright.LocatorTextContentOptions) (string, error)
	InnerHTML(options ...playwright.LocatorInnerHTMLOptions) (string, error)
	InnerText(options ...playwright.LocatorInnerTextOptions) (string, error)
	InputValue(options ...playwright.LocatorInputValueOptions) (string, error)
	IsChecked(options ...playwright.LocatorIsCheckedOptions) (bool, error)
	IsDisabled(options ...playwright.LocatorIsDisabledOptions) (bool, error)
	IsEditable(options ...playwright.LocatorIsEditableOptions) (bool, error)
	IsEnabled(options ...playwright.LocatorIsEnabledOptions) (bool, error)
	IsHidden(options ...playwright.LocatorIsHiddenOptions) (bool, error)
	IsVisible(options ...playwright.LocatorIsVisibleOptions) (bool, error)
	BoundingBox(options ...playwright.LocatorBoundingBoxOptions) (*playwright.Rect, error)
	Hover(options ...playwright.LocatorHoverOptions) error
	Click(options ...playwright.LocatorClickOptions) error
	Dblclick(options ...playwright.LocatorDblclickOptions) error
	Check(options ...playwright.LocatorCheckOptions) error
	Uncheck(options ...playwright.LocatorUncheckOptions) error
	Press(key string, options ...playwright.LocatorPressOptions) error
	Type(text string, options ...playwright.LocatorTypeOptions) error
	Fill(value string, options ...playwright.LocatorFillOptions) error
	Clear(options ...playwright.LocatorClearOptions) error
	SelectText(options ...playwright.LocatorSelectTextOptions) error
	SelectOption(values playwright.SelectOptionValues, options ...playwright.LocatorSelectOptionOptions) ([]string, error)
	SetInputFiles(files interface{}, options ...playwright.LocatorSetInputFilesOptions) error
	Focus(options ...playwright.LocatorFocusOptions) error
	Blur(options ...playwright.LocatorBlurOptions) error
	DragTo(target playwright.Locator, options ...playwright.LocatorDragToOptions) error
	Evaluate(expression string, arg interface{}, options ...playwright.LocatorEvaluateOptions) (interface{}, error)
	EvaluateAll(expression string, arg ...interface{}) (interface{}, error)
	EvaluateHandle(expression string, arg interface{}, options ...playwright.LocatorEvaluateHandleOptions) (playwright.JSHandle, error)
	ScrollIntoViewIfNeeded(options ...playwright.LocatorScrollIntoViewIfNeededOptions) error
	Screenshot(options ...playwright.LocatorScreenshotOptions) ([]byte, error)
	AriaSnapshot(options ...playwright.LocatorAriaSnapshotOptions) (string, error)
	WaitFor(options ...playwright.LocatorWaitForOptions) error
	Err() error
}
