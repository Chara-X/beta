/*
go test -v -p 1 -timeout 10m -run TestGPUInspectionCancel
aptsc -test.list "Test"
aptsc -test.v -test.parallel 1 -test.run "TestGPUInspectionCancel"
*/
package test

import (
	"crypto/tls"
	"embed"
	"flag"
	"net/http"
	"testing"

	"github.com/Chara-X/aptsc"
)

//go:embed testdata
var td embed.FS
var c *aptsc.Client

func TestMain(m *testing.M) {
	flag.Parse()
	c = aptsc.New(&http.Client{Transport: &http.Transport{TLSClientConfig: &tls.Config{InsecureSkipVerify: true}}}, "https://"+flag.Arg(0))
	m.Run()
}
