package playwright

import (
	"os/exec"

	"github.com/playwright-community/playwright-go"
)

// [playwright.PlaywrightDriver]
type PlaywrightDriver struct{ Version string }

// [playwright.NewDriver]
func NewDriver(options ...*playwright.RunOptions) (*PlaywrightDriver, error)

// [playwright.PlaywrightDriver.Install]
func (d *PlaywrightDriver) Install() error

// [playwright.PlaywrightDriver.Command]
func (d *PlaywrightDriver) Command(arg ...string) *exec.Cmd

// [playwright.PlaywrightDriver.Uninstall]
func (d *PlaywrightDriver) Uninstall() error
