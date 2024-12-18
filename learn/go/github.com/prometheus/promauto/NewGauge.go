package promauto

import "github.com/Chara-X/go-study/github.com/prometheus"

func NewGauge(opts struct {
	Name string
	Help string
	// ...
}) prometheus.Gauge
