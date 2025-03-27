package test

import (
	"fmt"
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
	"github.com/Chara-X/aptsc/util"
)

type modelInspectionCase struct {
	name    string
	timeout time.Duration
	req     *msg.ModelInspectionCreateRequest
}

func TestModelInspection(t *testing.T) {
	var v msg.GPUNodesGetResponse
	c.GetFromJson(aptsc.GpuNodesGETPath, &v)
	for _, n := range []int{1, 2, 4} {
		t.Run(fmt.Sprintf("%dnodes", n), func(t *testing.T) {
			for _, tt := range []modelInspectionCase{} {
				t.Run(tt.name, func(t *testing.T) {
					for _, tt.req.NodeList = range util.CombineN(v.Nodes, n) {
						if err := util.Try(func() {
							t.Log(tt.req.NodeList)
							c.PostAsJson(aptsc.ModelInspectionCreatePath, tt.req)
							defer c.Delete(aptsc.ModelInspectionDeletePath)
							var v msg.ModelInspectionGetResponse
							c.Polling(tt.timeout, func() bool {
								c.GetFromJson(aptsc.ModelInspectionGetPath, &v)
								return v.ModelTrainResults[0].Status == "running"
							})
							t.Log(v)
						}); err == nil {
							return
						}
					}
					t.Fatal("no available nodes")
				})
			}
		})
	}
}
