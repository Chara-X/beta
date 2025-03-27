package msg

type CCLInspectionCreateRequest struct {
	OutAlgbw    int               `json:"outAlgbw"`
	OutBusbw    int               `json:"outBusbw"`
	InAlgbw     int               `json:"inAlgbw"`
	InBusbw     int               `json:"inBusbw"`
	GpuPerNode  int               `json:"gpuPerNode"`
	TestScene   string            `json:"testScene"`
	NodeScene   string            `json:"nodeScene"`
	InspectType string            `json:"inspectType"`
	CclType     string            `json:"cclType"`
	Command     string            `json:"command"`
	NodeList    []string          `json:"nodeList"`
	CclParams   []VendorThreshold `json:"cclParams"`
}
type VendorThreshold struct {
	OutAlgbw   int    `json:"outAlgbw"`
	OutBusbw   int    `json:"outBusbw"`
	InAlgbw    int    `json:"inAlgbw"`
	InBusbw    int    `json:"inBusbw"`
	GpuPerNode int    `json:"gpuPerNode"`
	Command    string `json:"command"`
	GpuVendor  string `json:"gpuVendor"`
}

// {
// 	"inspectType": "BRCCL",
// 	"cclType": "AllReduce",
// 	"command": "-G 1 -b 512 -e 20m",
// 	"gpuPerNode": 8,
// 	"nodeScene": "cluster",
// 	"nodeList": [],
// 	"testScene": "cluster",
// 	"outAlgbw": 0,
// 	"outBusbw": 0,
// 	"inAlgbw": 0,
// 	"inBusbw": 0,
// 	"cclParams": [
// 	  {
// 		"gpuPerNode": 8,
// 		"gpuVendor": "BIREN",
// 		"command": "-G 1 -b 512 -e 20m",
// 		"inAlgbw": 0,
// 		"outAlgbw": 0,
// 		"inBusbw": 0,
// 		"outBusbw": 0
// 	  }
// 	]
// }
