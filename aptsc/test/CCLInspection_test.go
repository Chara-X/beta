package test

import (
	"fmt"
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
	"github.com/Chara-X/aptsc/util"
)

type cclInspectionCase struct {
	name    string
	timeout time.Duration
	req     *msg.CCLInspectionCreateRequest
}

func TestCCLInspection(t *testing.T) {
	var v msg.GPUNodesGetResponse
	c.GetFromJson(aptsc.GpuNodesGETPath, &v)
	for _, n := range []int{1} {
		t.Run(fmt.Sprintf("%dnodes", n), func(t *testing.T) {
			for _, tt := range []cclInspectionCase{
				{name: "cluster AllReduce 2GPUs", timeout: 2 * time.Minute, req: &msg.CCLInspectionCreateRequest{InspectType: "BRCCL", CclType: "AllReduce", Command: "-G 1 -b 512 -e 20m", GpuPerNode: 2, TestScene: "cluster"}},
				{name: "cluster AlltoAll 2GPUs", timeout: 2 * time.Minute, req: &msg.CCLInspectionCreateRequest{InspectType: "BRCCL", CclType: "AlltoAll", Command: "-G 1 -b 512 -e 20m", GpuPerNode: 2, TestScene: "cluster"}},
			} {
				t.Run(tt.name, func(t *testing.T) {
					for _, tt.req.NodeList = range util.CombineN(v.Nodes, n) {
						if err := util.Try(func() {
							t.Log(tt.req.NodeList)
							c.PostAsJson(aptsc.CCLInspectionCreatePath, tt.req)
							defer c.Post(aptsc.CCLInspectionDeletePath, nil)
							var v msg.CCLInspectionGetResponse
							c.Polling(tt.timeout, func() bool {
								c.GetFromJson(aptsc.CCLInspectionGetPath, &v)
								return v.Status == "running"
							})
							t.Log(v)
						}); err == nil {
							return
						} else {
							t.Log(err)
						}
					}
					t.Fatal("no available nodes")
				})
			}
		})
	}
}
