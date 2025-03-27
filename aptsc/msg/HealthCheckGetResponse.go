package msg

type HealthCheckGetResponse struct {
	Result    []HealthyCheckResult `json:"result"`
	Status    string               `json:"status"`
	StartTime string               `json:"startTime"`
	EndTime   string               `json:"endTime"`
	Vendor    string               `json:"vendor"`
	NodeNum   *int                 `json:"nodeNum"`
	GpuNum    *int                 `json:"gpuNum"`
	ParaNum   *int                 `json:"paraNum"`
	StoreNum  *int                 `json:"storeNum"`
}
type HealthyCheckResult struct {
	Object        string   `json:"object"`
	ObjectId      []string `json:"objectId"`
	CheckType     string   `json:"checkType"`
	CheckSubItem  string   `json:"checkSubItem"`
	SubResult     string   `json:"subResult"`
	CheckDetail   string   `json:"checkDetail"`
	CheckDetailEn string   `json:"checkDetailEn"`
	Status        string   `json:"status"`
	StartTime     string   `json:"startTime"`
	EndTime       string   `json:"endTime"`
	ShowDetail    bool     `json:"showDetail"`
}
