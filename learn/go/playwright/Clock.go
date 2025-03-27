package playwright

import "github.com/playwright-community/playwright-go"

// [playwright.Clock]
type Clock interface {
	SetFixedTime(time interface{}) error
	Install(options ...playwright.ClockInstallOptions) error
	PauseAt(time interface{}) error
	FastForward(ticks interface{}) error
	RunFor(ticks interface{}) error
	Resume() error
}
