package http

import "net/http"

func ServeFile(w ResponseWriter, r *http.Request, name string)
