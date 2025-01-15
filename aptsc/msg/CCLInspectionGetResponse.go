package msg

type CCLInspectionGetResponse struct {
	GpuNum    int     `json:"gpuNum"`
	NodeNum   int     `json:"nodeNum"`
	Status    string  `json:"status"`
	CclType   string  `json:"cclType"`
	TestScene string  `json:"testScene"`
	Result    string  `json:"result"`
	InAlgbw   float32 `json:"inAlgbw"`
	InBusbw   float32 `json:"inBusbw"`
	OutAlgbw  float32 `json:"outAlgbw"`
	OutBusbw  float32 `json:"outBusbw"`

	FailNodeList []string `json:"failNodeList"`
	FailNodeNum  int      `json:"failNodeNum"`
	PassNodeNum  int      `json:"passNodeNum"`
}
