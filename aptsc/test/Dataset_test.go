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
	var v msg.DatasetGetResponse
	c.GetFromJson(aptsc.DatasetGetPath, &v)
	for _, tt := range []datasetCase{
		{name: "tar ShareStorage", file: "testdata/wudao.tar", req: &msg.DatasetBindRequest{DatasetFileName: "wudao.tar", DatasetName: "wudao9", Pvc: v.Datasets[0].Pvc, IsShareStorage: true, Namespace: v.Datasets[0].NameSpace}},
		{name: "tar.gz ShareStorage", file: "testdata/wudao.tar.gz", req: &msg.DatasetBindRequest{DatasetFileName: "wudao.tar.gz", DatasetName: "wudao9", Pvc: v.Datasets[0].Pvc, IsShareStorage: true, Namespace: v.Datasets[0].NameSpace}},
		{name: "zip ShareStorage", file: "testdata/wudao.zip", req: &msg.DatasetBindRequest{DatasetFileName: "wudao.zip", DatasetName: "wudao9", Pvc: v.Datasets[0].Pvc, IsShareStorage: true, Namespace: v.Datasets[0].NameSpace}},
	} {
		t.Run(tt.name, func(t *testing.T) {
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
			func() {
				c.PostFile(aptsc.DatasetCreatePath, td, tt.file)
				defer c.Post(fmt.Sprintf(aptsc.DatasetDeletePath, tt.req.DatasetName), nil)
				c.PostAsJson(aptsc.DatasetBindPath, tt.req)
				defer c.Post(fmt.Sprintf(aptsc.DatasetUnbindPath, tt.req.DatasetName), nil)
				c.Polling(5*time.Minute, f("running"))
				t.Log(v)
			}()
			c.Polling(5*time.Minute, f("deleting"))
			t.Log(v)
		})
	}
}
func TestConfigmapPersistence(t *testing.T) {
	var v msg.DatasetGetResponse
	c.GetFromJson(aptsc.DatasetGetPath, &v)
	t.Log(v)
	if len(v.Datasets) == 0 {
		t.Fatal("configmap persistence failed")
	}
}
