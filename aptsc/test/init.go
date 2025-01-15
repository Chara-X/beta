/*
go test -timeout 30m github.com/Chara-X/aptsc/test -v -p 1
*/
package test

import (
	"crypto/tls"
	"net/http"

	"github.com/Chara-X/aptsc"
)

var c = aptsc.New(&http.Client{Transport: &http.Transport{TLSClientConfig: &tls.Config{InsecureSkipVerify: true}}}, "https://10.166.209.110")

func init() {}
