package playwright

import "github.com/playwright-community/playwright-go"

var _ playwright.EventEmitter

// [playwright.EventEmitter]
type EventEmitter interface {
	Emit(name string, payload ...interface{}) bool
	On(name string, handler interface{})
	RemoveListener(name string, handler interface{})
}
