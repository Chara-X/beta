package test

import (
	"fmt"
	"testing"
	"time"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

type datasetCase struct {
	name string
	file string
	req  *msg.DatasetBindRequest
}

func TestDataset(t *testing.T) {
	for _, tt := range []datasetCase{
		{name: "zip", file: "./wudao/wudao.zip", req: &msg.DatasetBindRequest{DatasetFileName: "wudao.zip", DatasetName: "wudao9", Pvc: "llama70b", IsShareStorage: true, Namespace: "test70b"}},
	} {
		t.Run(tt.name, func(t *testing.T) {
			c.PostFile(aptsc.DatasetCreatePath, tt.file)
			c.PostAsJson(aptsc.DatasetBindPath, tt.req)
			var v msg.DatasetGetResponse
			var f = func(status string) func() bool {
				return func() bool {
					c.GetFromJson(aptsc.DatasetGetPath, &v)
					for _, d := range v.Datasets {
						if d.DatasetName == tt.req.DatasetName && d.Status == status {
							return true
						}
					}
					return false
				}
			}
			c.Polling(5*time.Minute, f("running"))
			t.Log(v)
			c.Post(fmt.Sprintf(aptsc.DatasetUnbindPath, tt.req.DatasetName), nil)
			c.Post(fmt.Sprintf(aptsc.DatasetDeletePath, tt.req.DatasetName), nil)
			c.Polling(5*time.Minute, f("deleting"))
			t.Log(v)
		})
	}
}
