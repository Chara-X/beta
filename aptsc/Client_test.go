package aptsc

import (
	"net/http"
	"testing"
	"time"
)

func TestClient_Polling(t *testing.T) {
	type fields struct {
		c   *http.Client
		srv string
	}
	type args struct {
		timeout time.Duration
		f       func() bool
	}
	tests := []struct {
		name   string
		fields fields
		args   args
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			c := &Client{
				c:   tt.fields.c,
				srv: tt.fields.srv,
			}
			c.Polling(tt.args.timeout, tt.args.f)
		})
	}
}
