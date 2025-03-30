package promauto

import "github.com/Chara-X/go-study/prometheus"

func NewGauge(opts struct {
	Name string
	Help string
	// ...
}) prometheus.Gauge
