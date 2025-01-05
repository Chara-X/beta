package playwright

import (
	"os/exec"

	"github.com/playwright-community/playwright-go"
)

type PlaywrightDriver struct {
	d       *playwright.PlaywrightDriver
	Version string
}

func NewDriver(options ...*playwright.RunOptions) (*PlaywrightDriver, error) {
	var d, err = playwright.NewDriver(options...)
	return &PlaywrightDriver{d: d, Version: d.Version}, err
}
func (d *PlaywrightDriver) Install() error                  { return d.d.Install() }
func (d *PlaywrightDriver) Command(arg ...string) *exec.Cmd { return d.d.Command(arg...) }
func (d *PlaywrightDriver) Uninstall() error                { return d.d.Uninstall() }
