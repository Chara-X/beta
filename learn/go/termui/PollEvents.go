package termui

import "github.com/gizak/termui/v3"

func PollEvents() <-chan termui.Event { return termui.PollEvents() }
