package msg

type GPUInspectionGetResponse struct {
	DiagCreated    string `json:"diag_created"`
	DiagGpuOutputs []struct {
		DiagExecErrmsg   string   `json:"diag_exec_errmsg"`
		DiagFailCount    int      `json:"diag_fail_count"`
		GpuUUID          string   `json:"gpuUUID"`
		GpuResultSummary struct{} `json:"gpu_result_summary"`
		Model            string   `json:"model"`
		NodeName         string   `json:"node_name"`
		Result           string   `json:"result"`
		Vendor           string   `json:"vendor"`
	} `json:"diag_gpu_outputs"`
	DiagTotalCount    int    `json:"diag_total_count"`
	InspectType       string `json:"inspect_type"`
	Status            string `json:"status"`
	TaskResultSummary struct {
		TotalCards  int `json:"totalCards"`
		TotalChecks int `json:"totalChecks"`
		TotalUsed   int `json:"totalUsed"`
	} `json:"task_result_summary"`
}
