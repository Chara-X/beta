package prometheus

import "github.com/prometheus/client_golang/prometheus"

type Gauge interface {
	prometheus.Metric
	Inc()
	Dec()
	Set(float64)
	Add(float64)
	Sub(float64)
	SetToCurrentTime()
}
