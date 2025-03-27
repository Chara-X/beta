package msg

type VCJobEventsGetResponse struct {
	Name              string              `json:"name"`
	NameSpace         string              `json:"namespace"`
	State             string              `json:"state"`
	Phase             string              `json:"phase"`
	CreationTimestamp string              `json:"creationTimestamp"`
	Duration          string              `json:"duration"`
	MinAvailable      int32               `json:"minAvaliable,omitempty"`
	Running           int32               `json:"running,omitempty"`
	UID               string              `json:"uid,omitempty"`
	Conditions        []Condition         `json:"conditions"`
	JobInfo           []ResourceEventInfo `json:"jobWarningInfo,omitempty"`
	PodInfo           []ResourceEventInfo `json:"podWarningInfo,omitempty"`
	GpuInfo           []ResourceEventInfo `json:"gpuWarningInfo,omitempty"`
	RdmaDeviceInfo    []ResourceEventInfo `json:"rdmaDeviceWarningInfo,omitempty"`
	GpuTrainInfo      []ResourceEventInfo `json:"gpuTrainInfo,omitempty"`
}
type Condition struct {
	Status             string `json:"status"`
	LastTransitionTime string `json:"lastTransitionTime"`
}
type ResourceEventInfo struct {
	TimeStamp  string `json:"timeStamp"`
	JobName    string `json:"jobName,omitempty"`
	ObjectType string `json:"objectType"`
	Object     string `json:"object"`
	NodeName   string `json:"nodeName"`
	Reason     string `json:"reason"`
	// Kind       string `json:"kind"`
	Message    string `json:"message"`
	EventName  string `json:"eventName,omitempty"`
	Suggestion string `json:"suggestion,omitempty"`
}
