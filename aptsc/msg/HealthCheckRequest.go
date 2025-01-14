package msg

type HealthCheckRequest struct {
	Object         string            `json:"object"`
	CheckSubInfo   CheckSubInfo      `json:"checkSubInfo"`
	HealthCheckCfg HealthCheckConfig `json:"healthCheckCfg"`
}
type CheckSubInfo struct {
	GpuConsistencyCheck  []GpuConsistencyCheck  `json:"gpuConsistencyCheck,omitempty"`
	RdmaConsistencyCheck []RdmaConsistencyCheck `json:"rdmaConsistencyCheck,omitempty"`
}
type GpuConsistencyCheck string

const (
	GpuHardDropNum GpuConsistencyCheck = "GpuHardDropNum"
	GpuSoftDropNum GpuConsistencyCheck = "GpuSoftDropNum"
	GpuTopo        GpuConsistencyCheck = "GpuTopo"
	GpuDriver      GpuConsistencyCheck = "GpuDriver"
	GpuLinkSpeed   GpuConsistencyCheck = "GpuLinkSpeed"
)

type RdmaConsistencyCheck string

const (
	RdmaPortNum  RdmaConsistencyCheck = "RdmaPortNum"
	RdmaMtu      RdmaConsistencyCheck = "RdmaMtu"
	RdmaParaVer  RdmaConsistencyCheck = "RdmaParaVer"
	RdmaStoreVer RdmaConsistencyCheck = "RdmaStoreVer"
	RdmaResource RdmaConsistencyCheck = "RdmaResource"
)

type HealthCheckConfig struct {
	RdmaNums uint32 `json:"rdmaNums"`
	GpuNums  uint32 `json:"gpuNums"`
}
