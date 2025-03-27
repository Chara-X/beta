package msg

type HealthCheckCreateRequest struct {
	Object         string         `json:"object"`
	ObjectList     []string       `json:"objectList"`
	CheckSubInfo   CheckSubInfo   `json:"checkSubInfo"`
	HealthCheckCfg HealthCheckCfg `json:"healthCheckCfg"`
}
type CheckSubInfo struct {
	GpuConsistencyCheck   []string `json:"gpuConsistencyCheck"`
	RdmaConsistencyCheck  []string `json:"rdmaConsistencyCheck"`
	GpuAvailabilityCheck  []string `json:"gpuAvailabilityCheck"`
	RdmaAvailabilityCheck []string `json:"rdmaAvailabilityCheck"`
	ServerVerCheck        []string `json:"serverVerCheck"`
}
type HealthCheckCfg struct {
	GpuNums              *int          `json:"gpuNums"`
	RdmaNums             *int          `json:"rdmaNums"`
	GpuDriverVer         VendorGpuInfo `json:"gpuDriverVer"`
	RdmaParamDriverVer   string        `json:"rdmaParamDriverVer"`
	RdmaStorageDriverVer string        `json:"rdmaStorageDriverVer"`
	RdmaParamFwVer       string        `json:"rdmaParamFwVer"`
	RdmaStorageFwVer     string        `json:"rdmaStorageFwVer"`
	OsVer                string        `json:"osVer"`
	ServerFwVer          string        `json:"serverFwVer"`
	BmcVer               string        `json:"bmcVer"`
	BiosVer              string        `json:"biosVer"`
	EpldVer              string        `json:"epldVer"`
}
type VendorGpuInfo struct {
	NwVendor  map[string][]string `json:"nwVendor,omitempty"`
	BrVendor  map[string][]string `json:"brVendor,omitempty"`
	HwjVendor map[string][]string `json:"hwjVendor,omitempty"`
}
