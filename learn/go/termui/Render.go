package termui

import (
	"github.com/gizak/termui/v3"
	_ "github.com/gizak/termui/v3/widgets"
)

func Render(items ...termui.Drawable) { termui.Render(items...) }
