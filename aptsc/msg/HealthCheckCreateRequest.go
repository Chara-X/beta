package msg

type HealthCheckCreateRequest struct {
	Object         string            `json:"object"`
	CheckSubInfo   CheckSubInfo      `json:"checkSubInfo"`
	HealthCheckCfg HealthCheckConfig `json:"healthCheckCfg"`
}
type CheckSubInfo struct {
	GpuConsistencyCheck  []string `json:"gpuConsistencyCheck,omitempty"`
	RdmaConsistencyCheck []string `json:"rdmaConsistencyCheck,omitempty"`
}
type HealthCheckConfig struct {
	RdmaNums uint32 `json:"rdmaNums"`
	GpuNums  uint32 `json:"gpuNums"`
}
