package test

import (
	"testing"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

func TestCCLRDMAInspectionConfig(t *testing.T) {
	var v msg.CCLRDMAInspectionConfigGetResponse
	c.GetFromJson(aptsc.CCLRDMAInspectionConfigGetPath, &v)
	t.Log(v)
}
