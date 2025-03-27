package msg

type ModelInspectionGetResponse struct {
	ModelTrainResults []AiModelTrainResultSpec
}
type AiModelTrainResultSpec struct {
	TaskId         string `json:"taskId"`
	ModelName      string `json:"modelName"`
	ModelImageName string `json:"modelImageName"`
	GpuFactory     string `json:"gpuFactory"`
	GpuTotal       int64  `json:"gpuTotal"`
	CpuPerNode     int64  `json:"cpuPerNode"`
	GpuPerNode     int64  `json:"gpuPerNode"`
	MemoryPerNode  int64  `json:"memoryPerNode"`
	SuperParam     string `json:"superParam"`
	NodeNum        int64  `json:"nodeNum"`
	ExceResult     string `json:"exceResult"`
	Status         string `json:"status"`
	TestResult     string `json:"testResult"`
	TestResultZh   string `json:"testResultZh"`
	TestScene      string `json:"testScene"`
	StartTime      string `json:"startTime"`
	EndTime        string `json:"endTime"`
	Pvc            string `json:"pvc"`
	DatasetName    string `json:"datasetName"`
}
