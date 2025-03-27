package msg

type ModelInspectionConfigGetResponse struct {
	CodePath            string `json:"codePath"`
	FirstStepTimeout    int    `json:"firstStepTimeout"`
	GeneralShellPath    string `json:"generalShellPath"`
	ModelImageName      string `json:"modelImageName"`
	ModelName           string `json:"modelName"`
	NonFirstStepTimeout int    `json:"nonFirstStepTimeout"`
	PerfOptShellPath    string `json:"perfOptShellPath"`
}
