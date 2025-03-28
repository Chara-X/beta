package dns

import (
	"github.com/miekg/dns"
)

type Msg struct {
	dns.MsgHdr
	Question []dns.Question
	Answer   []dns.RR
	Ns       []dns.RR
	Extra    []dns.RR
}

func (m *Msg) Unpack(msg []byte) error
func (m *Msg) Pack() ([]byte, error)
func (m *Msg) SetReply(request *Msg)
