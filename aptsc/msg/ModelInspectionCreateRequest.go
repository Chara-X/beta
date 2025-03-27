package msg

type ModelInspectionCreateRequest struct {
	GpuTotal              int      `json:"gpuTotal,omitempty"`
	GpuPerNode            int      `json:"gpuPerNode,omitempty"`
	TaskId                string   `json:"taskId,omitempty"`
	NodeList              []string `json:"nodeList,omitempty"`
	SuperParam            string   `json:"superParam,omitempty"`
	Cmd                   string   `json:"cmd,omitempty"`
	CpuPerNode            int      `json:"cpuPerNode,omitempty"`
	MemoryPerNode         int      `json:"memoryPerNode,omitempty"`
	GpuFactory            string   `json:"gpuFactory,omitempty"`
	PvcEnable             string   `json:"pvcEnable,omitempty"`
	PvcSize               string   `json:"pvcSize,omitempty"`
	PvcNamespace          string   `json:"pvcNamespace,omitempty"`
	PvcName               string   `json:"pvc,omitempty"`
	DatasetName           string   `json:"datasetName,omitempty"`
	ModelImageName        string   `json:"modelImageName,omitempty"`
	CodePath              string   `json:"codePath,omitempty"`
	GeneralShellPath      string   `json:"generalShellPath,omitempty"`
	PerfOptShellPath      string   `json:"perfOptShellPath,omitempty"`
	ShellPath             string   `json:"shellPath,omitempty"`
	ModelName             string   `json:"modelName,omitempty"`
	TestScene             string   `json:"testScene,omitempty"`
	IsConfirmedModelTrain bool     `json:"isConfirmedModelTrain,omitempty"`
	FirstStepTimeout      int      `json:"firstStepTimeout,omitempty"`
	NonFirstStepTimeout   int      `json:"nonFirstStepTimeout,omitempty"`
	CreatedAt             string   `json:"createdAt,omitempty"`
	StopAt                int      `json:"stopAt,omitempty"`
	LogNameNodeNameInfo   string   `json:"logNameNodeNameInfo,omitempty"`
}
