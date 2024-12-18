package httputil

import "net/http"

func DumpResponse(response *http.Response, body bool) ([]byte, error)
