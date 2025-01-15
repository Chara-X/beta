package msg

type CCLInspectionCreateRequest struct {
	InspectType string   `json:"inspectType"`
	CCLType     string   `json:"cclType"`
	Command     string   `json:"command"`
	GPUPerNode  int      `json:"gpuPerNode"`
	NodeList    []string `json:"nodeList"`
	TestScene   string   `json:"testScene"`
	OutAlgbw    float64  `json:"outAlgbw"`
	OutBusbw    float64  `json:"outBusbw"`
	InAlgbw     float64  `json:"inAlgbw"`
	InBusbw     float64  `json:"inBusbw"`
}
