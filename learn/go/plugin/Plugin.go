package plugin

import "plugin"

type Plugin struct{ p *plugin.Plugin }

func Open(path string) (*Plugin, error) {
	var p, err = plugin.Open(path)
	return &Plugin{p}, err
}
func (p *Plugin) Lookup(symName string) (interface{}, error) { return p.p.Lookup(symName) }
