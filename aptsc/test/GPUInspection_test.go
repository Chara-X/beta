package test

import (
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

type gpuInspectionCase struct {
	name    string
	timeout time.Duration
	req     *msg.GPUInspectionCreateRequest
}

func TestGPUInspectionContinue(t *testing.T) {
	for _, tt := range []gpuInspectionCase{
		{name: "short", timeout: 5 * time.Minute, req: &msg.GPUInspectionCreateRequest{DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "short"}}, InspectCount: 1, InspectType: "GPU"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.GPUInspectionCreatePath, tt.req)
			var v msg.GPUInspectionGetResponse
			c.Polling(tt.timeout, func() bool {
				c.GetFromJson(aptsc.GPUInspectionGetPath, &v)
				return v.Status == "running"
			})
			t.Log(v)
		})
	}
}
func TestGPUInspectionCancel(t *testing.T) {
	for _, tt := range []gpuInspectionCase{
		{name: "short", timeout: 5 * time.Minute, req: &msg.GPUInspectionCreateRequest{DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "short"}}, InspectCount: 1, InspectType: "GPU"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.GPUInspectionCreatePath, tt.req)
			c.Delete(aptsc.GPUInspectionCreatePath)
		})
	}
}
