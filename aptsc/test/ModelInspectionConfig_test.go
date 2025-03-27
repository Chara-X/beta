package test

import (
	"testing"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

func TestModelInspectionConfig(t *testing.T) {
	var v msg.ModelInspectionConfigGetResponse
	c.GetFromJson(aptsc.ModelInspectionConfigGetPath, &v)
	t.Log(v)
}
