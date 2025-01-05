package playwright

import "github.com/playwright-community/playwright-go"

type PlaywrightAssertions interface {
	Page(page playwright.Page) playwright.PageAssertions
	Locator(locator playwright.Locator) playwright.LocatorAssertions
	APIResponse(response playwright.APIResponse) playwright.APIResponseAssertions
}

func NewPlaywrightAssertions(timeout ...float64) PlaywrightAssertions {
	return &playwrightAssertions{pa: playwright.NewPlaywrightAssertions(timeout...)}
}

type playwrightAssertions struct {
	pa playwright.PlaywrightAssertions
}

func (pa *playwrightAssertions) Page(page playwright.Page) playwright.PageAssertions {
	return pa.pa.Page(page)
}
func (pa *playwrightAssertions) Locator(locator playwright.Locator) playwright.LocatorAssertions {
	return pa.pa.Locator(locator)
}
func (pa *playwrightAssertions) APIResponse(response playwright.APIResponse) playwright.APIResponseAssertions {
	return pa.pa.APIResponse(response)
}
