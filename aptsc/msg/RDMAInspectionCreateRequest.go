package msg

type RDMAInspectionCreateRequest struct {
	TestScene          string   `json:"testScene"`
	NodeList           []string `json:"nodeList"`
	RDMAPerNode        int      `json:"rdmaPerNode"`
	BandWidthThreshold int      `json:"bandWidthThreshold"`
}
