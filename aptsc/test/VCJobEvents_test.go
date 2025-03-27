package test

import (
	"fmt"
	"testing"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

func TestVCJobEvents(t *testing.T) {
	var v msg.VCJobEventsGetResponse
	c.GetFromJson(fmt.Sprintf(aptsc.VCJobEventsGetPath, "icf-test", "inp12793634817229776632-js"), &v)
	t.Log(v)
}
