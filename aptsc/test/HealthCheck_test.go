package test

import (
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
	"github.com/Chara-X/aptsc/util"
)

type healthCheckCase struct {
	name string
	req  *msg.HealthCheckCreateRequest
}

func TestHealthCheck(t *testing.T) {
	for _, tt := range []healthCheckCase{
		{name: "gpu hard drop", req: &msg.HealthCheckCreateRequest{Object: "cluster", CheckSubInfo: msg.CheckSubInfo{GpuConsistencyCheck: []string{"gpuharddropnum"}}, HealthCheckCfg: msg.HealthCheckCfg{GpuNums: util.Int(10)}}},
		{name: "rdma port num", req: &msg.HealthCheckCreateRequest{Object: "cluster", CheckSubInfo: msg.CheckSubInfo{RdmaConsistencyCheck: []string{"rdmaportnum"}}, HealthCheckCfg: msg.HealthCheckCfg{RdmaNums: util.Int(10)}}},
		{name: "gpu topo", req: &msg.HealthCheckCreateRequest{Object: "cluster", CheckSubInfo: msg.CheckSubInfo{GpuConsistencyCheck: []string{"gputopo"}}}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.HealthCheckCreatePath, tt.req)
			var v msg.HealthCheckGetResponse
			c.Polling(time.Minute, func() bool {
				c.GetFromJson(aptsc.HealthCheckGetPath, &v)
				return v.Status == "checking"
			})
			t.Log(v)
		})
	}
}
