package test

import (
	"testing"

	"github.com/Chara-X/aptsc"
	"github.com/Chara-X/aptsc/msg"
)

func TestNamespaceConfig(t *testing.T) {
	var v msg.NamespaceConfigGetResponse
	c.GetFromJson(aptsc.NamespaceConfigGetPath, &v)
	t.Log(v)
}
