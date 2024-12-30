package playwright

import "github.com/playwright-community/playwright-go"

func Install(options ...*playwright.RunOptions) error {
	return playwright.Install(options...)
}
