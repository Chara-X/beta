package msg

type CCLInspectionGetResponse struct {
	TotalNodeNum   int                      `json:"totalNodeNum"`
	TotalGpuNum    int                      `json:"totalGpuNum"`
	TotalVendorNum int                      `json:"totalVendorNum"`
	FailVendorNum  int                      `json:"failVendorNum"`
	FailNodeNum    int                      `json:"failNodeNum"`
	Status         string                   `json:"status"`
	EndTime        string                   `json:"endTime"`
	NodeScene      string                   `json:"nodeScene"`
	CclResult      []map[string]interface{} `json:"cclResult"`
}
