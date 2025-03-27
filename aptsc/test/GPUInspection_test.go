package test

import (
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/assert"
	"github.com/Chara-X/aptsc/msg"
)

type gpuInspectionCase struct {
	name string
	req  *msg.GPUInspectionCreateRequest
}

func TestGPUInspectionContinue(t *testing.T) {
	for _, tt := range []gpuInspectionCase{
		{name: "quick", req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:quick", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "quick"}}, InspectCount: 1, InspectType: "GPU"}},
		{name: "medium", req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:medium", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "medium"}}, InspectCount: 1, InspectType: "GPU"}},
		// {name: "long", timeout: 8 * time.Minute, req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:long", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "long"}}, InspectCount: 1, InspectType: "GPU"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.GPUInspectionCreatePath, tt.req)
			var v msg.GPUInspectionGetResponse
			// defer c.Delete(aptsc.GPUInspectionDeletePath)
			c.Polling(time.Minute*8, func() bool {
				c.GetFromJson(aptsc.GPUInspectionGetPath, &v)
				return v.Status == "running"
			})
			t.Log(v)
			assert.AfterGPUInspectionFinish(&v)
		})
		time.Sleep(3 * time.Second) // panic: {"code":400,"err":"gpu inspection task is running"}
	}
}
func TestGPUInspectionCancel(t *testing.T) {
	for _, tt := range []gpuInspectionCase{
		{name: "quick", req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:quick", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "quick"}}, InspectCount: 1, InspectType: "GPU"}},
		{name: "medium", req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:medium", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "medium"}}, InspectCount: 1, InspectType: "GPU"}},
		{name: "long", req: &msg.GPUInspectionCreateRequest{DiagLevel: "biren:long", DiagConfigs: []*msg.DiagConfig{{Vendor: "biren", DiagLevel: "long"}}, InspectCount: 1, InspectType: "GPU"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.GPUInspectionCreatePath, tt.req)
			time.Sleep(1 * time.Second)
			c.Delete(aptsc.GPUInspectionDeletePath)
			var v msg.GPUInspectionGetResponse
			c.Polling(time.Minute*8, func() bool {
				c.GetFromJson(aptsc.GPUInspectionGetPath, &v)
				return v.Status == "running"
			})
			t.Log(v)
		})
	}
}
