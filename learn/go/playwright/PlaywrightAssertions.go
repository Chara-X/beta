package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.PlaywrightAssertions]
type PlaywrightAssertions interface {
	Page(page playwright.Page) playwright.PageAssertions
	Locator(locator playwright.Locator) playwright.LocatorAssertions
}

// [playwright.NewPlaywrightAssertions]
func NewPlaywrightAssertions(timeout ...float64) PlaywrightAssertions
