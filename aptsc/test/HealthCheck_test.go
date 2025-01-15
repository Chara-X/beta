package test

import (
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

type healthCheckCase struct {
	name string
	req  *msg.HealthCheckCreateRequest
}

func TestHealthCheck(t *testing.T) {
	for _, tt := range []healthCheckCase{
		{name: "gpu hard drop num and rdma port num", req: &msg.HealthCheckCreateRequest{Object: "cluster", CheckSubInfo: msg.CheckSubInfo{GpuConsistencyCheck: []string{"gpuharddropnum"}, RdmaConsistencyCheck: []string{"rdmaportnum"}}, HealthCheckCfg: msg.HealthCheckConfig{RdmaNums: 10, GpuNums: 10}}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.HealthCheckCreatePath, tt.req)
			var v msg.HealthCheckGetResponse
			c.Polling(3*time.Minute, func() bool {
				c.GetFromJson(aptsc.HealthCheckGetPath, &v)
				return v.Status == "checking"
			})
			t.Log(v)
		})
	}
}
