package httputil

import "net/http"

func DumpRequest(request *http.Request, body bool) ([]byte, error)
