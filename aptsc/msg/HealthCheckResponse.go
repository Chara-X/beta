package msg

import "time"

type HealthCheckResponse struct {
	Result    []CheckResult `json:"result"`
	Status    string        `json:"status"`
	StartTime time.Time     `json:"startTime"`
	EndTime   time.Time     `json:"endTime"`
	Vendor    string        `json:"vendor"`
	NodeNum   uint32        `json:"nodeNum"`
	GpuNum    uint32        `json:"gpuNum"`
	ParaNum   *uint32       `json:"paraNum,omitempty"`
	StoreNum  *uint32       `json:"storeNum,omitempty"`
}

type CheckResult struct {
	Object        string    `json:"object"`
	ObjectID      []string  `json:"objectId"`
	CheckType     string    `json:"checkType"`
	CheckSubItem  string    `json:"checkSubItem"`
	SubResult     string    `json:"subResult"`
	CheckDetail   string    `json:"checkDetail"`
	CheckDetailEn string    `json:"checkDetailEn"`
	Status        string    `json:"status"`
	StartTime     time.Time `json:"startTime"`
	EndTime       time.Time `json:"endTime"`
	ShowDetail    bool      `json:"showDetail"`
}
