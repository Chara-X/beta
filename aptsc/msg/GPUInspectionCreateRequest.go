package msg

type GPUInspectionCreateRequest struct {
	DiagLevel    string        `json:"diag_level"`
	DiagConfigs  []*DiagConfig `json:"diag_configs"`
	InspectCount int           `json:"inspect_count"`
	InspectType  string        `json:"inspect_type"`
	Mos          []string      `json:"mos"`
	Notes        []string      `json:"notes"`
}
type DiagConfig struct {
	Vendor    string `json:"vendor"`
	DiagParam string `json:"diag_param"`
	DiagLevel string `json:"diag_level"`
}
