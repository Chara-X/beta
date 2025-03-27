package msg

type GPUInspectionGetResponse struct {
	Inspect_type        string               `json:"inspect_type,omitempty"`
	Diag_level          string               `json:"diag_level,omitempty"`
	Diag_created        string               `json:"diag_created,omitempty"`
	Diag_finished       string               `json:"diag_finished,omitempty"`
	Diag_duration       string               `json:"diag_duration,omitempty"`
	Diag_total_count    int                  `json:"diag_total_count,omitempty"`
	Diag_success_count  int                  `json:"diag_success_count,omitempty"`
	Diag_fail_count     int                  `json:"diag_fail_count,omitempty"`
	Status              string               `json:"status,omitempty"`
	Result              string               `json:"result,omitempty"`
	Task_result_summary GpuInspectionSummary `json:"task_result_summary,omitempty"`
	Diag_gpu_outputs    []DiagGpuInfo        `json:"diag_gpu_outputs,omitempty"`
	ErrInfo             string               `json:"errInfo,omitempty"`
	ErrInfoZh           string               `json:"errInfoZh,omitempty"`
	Diag_total_nodes    int                  `json:"diag_total_nodes,omitempty"`
	DiagType            string               `json:"diagtype,omitempty"`
	InspectionNums      int                  `json:"inspection_nums,omitempty"`
}
type GpuInspectionSummary struct {
	TotalCards      int `json:"totalCards,omitempty"`
	TotalChecks     int `json:"totalChecks,omitempty"`
	TotalUsed       int `json:"totalUsed,omitempty"`
	CheckedCards    int `json:"checkedCards,omitempty"`
	HealthyChecks   int `json:"healthyChecks,omitempty"`
	UnhealthyChecks int `json:"unhealthyChecks,omitempty"`
	FailChecks      int `json:"failChecks,omitempty"`
}
type DiagGpuInfo struct {
	Diag_task_id       string           `json:"diag_task_id,omitempty"`
	Diag_fail_count    int              `json:"diag_fail_count,omitempty"`
	Diag_name          string           `json:"diag_name,omitempty"`
	GpuUUID            string           `json:"gpuUUID,omitempty"`
	GpuName            string           `json:"gpuName,omitempty"`
	Vendor             string           `json:"vendor,omitempty"`
	Model              string           `json:"model,omitempty"`
	Node_id            string           `json:"node_id,omitempty"`
	Node_name          string           `json:"node_name,omitempty"`
	Time_duration      int              `json:"time_duration,omitempty"`
	Diag_duration      string           `json:"diag_duration,omitempty"`
	Status             string           `json:"status,omitempty"`
	Diag_exec_succ     bool             `json:"diag_exec_succ,omitempty"`
	Diag_exec_errmsg   string           `json:"diag_exec_errmsg,omitempty"`
	Gpu_result_summary GPUResultSummary `json:"gpu_result_summary,omitempty"`
	Outputs            []DiagOutput     `json:"outputs,omitempty"`
	Result             string           `json:"result,omitempty"`
	Diaglevel          string           `json:"diaglevel,omitempty"`
	Slot               string           `json:"slot,omitempty"`
}
type GPUResultSummary struct {
	Success int    `json:"succ,omitempty"`
	Fail    int    `json:"fail,omitempty"`
	Result  string `json:"result,omitempty"`
}
type DiagOutput struct {
	Diag_gpu_outputs string `json:"diag_item_name,omitempty"`
	Diag_item_result string `json:"diag_item_result,omitempty"`
	Description      string `json:"description,omitempty"`
}
