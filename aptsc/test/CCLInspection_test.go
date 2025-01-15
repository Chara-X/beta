package test

import (
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

type cclInspectionCase struct {
	name    string
	timeout time.Duration
	req     *msg.CCLInspectionCreateRequest
}

func TestCCLInspectionContinue(t *testing.T) {
	for _, tt := range []cclInspectionCase{
		{name: "cluster AllReduce 1 nodes * 4 GPUs", timeout: 3 * time.Minute, req: &msg.CCLInspectionCreateRequest{InspectType: "BRCCL", CCLType: "AllReduce", Command: "-G 1 -b 512 -e 20m", GPUPerNode: 4, NodeList: []string{"cluster-cim2-minion-0-1"}, TestScene: "cluster"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostAsJson(aptsc.CCLInspectionCreatePath, tt.req)
			var v msg.CCLInspectionGetResponse
			c.Polling(tt.timeout, func() bool {
				c.GetFromJson(aptsc.CCLInspectionGetPath, &v)
				return v.Status == "running"
			})
			t.Log(v)
		})
	}
}
