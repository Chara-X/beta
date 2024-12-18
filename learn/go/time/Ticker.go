package time

import "time"

type Ticker struct {
	t *time.Ticker
	C <-chan Time
}

func NewTicker(d time.Duration) *Ticker { return &Ticker{t: time.NewTicker(d)} }
func (t *Ticker) Reset(d time.Duration) { t.t.Reset(d) }
func (t *Ticker) Stop()                 { t.t.Stop() }
