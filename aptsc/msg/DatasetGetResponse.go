package msg

type DatasetGetResponse struct {
	Datasets []DataSet `json:"dataSets"`
}
type DataSet struct {
	DatasetName     string `json:"datasetName"`
	DatasetSubfiles string `json:"datasetSubfiles"`
	ErrorInfo       string `json:"errorInfo"`
	NameSpace       string `json:"nameSpace"`
	Pvc             string `json:"pvc"`
	Status          string `json:"status"`
}

func (r *DatasetGetResponse) HasStatus(datasetName, status string) bool {
	for _, d := range r.Datasets {
		if d.DatasetName == datasetName && d.Status == status {
			return true
		}
	}
	return false
}
