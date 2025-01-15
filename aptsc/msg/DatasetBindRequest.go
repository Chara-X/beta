package msg

type DatasetBindRequest struct {
	DatasetFileName string `json:"datasetFileName"`
	DatasetName     string `json:"datasetName"`
	Pvc             string `json:"pvc"`
	IsShareStorage  bool   `json:"isShareStorage"`
	Namespace       string `json:"namespace"`
}
