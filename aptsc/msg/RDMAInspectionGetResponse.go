package msg

type RdmaInspectionGetResponse struct {
	TestRdmaResultList []*RdmaResult `json:"testRdmaResultList,omitempty"`
	TestScene          string        `json:"testScene,omitempty"`
	Status             string        `json:"status,omitempty"`
	TestRdmaResult     string        `json:"testRdmaResult,omitempty"`
	ErrInfo            string        `json:"errInfo,omitempty"`
	ErrInfoZh          string        `json:"errInfoZh,omitempty"`
	TestNodeNum        int           `json:"testNodeNum,omitempty"`
	TestRdmaNum        int           `json:"testRdmaNum,omitempty"`
	BandWidthThreshold int           `json:"bandWidthThreshold"`
}
type RdmaResult struct {
	LocalNodeName       string  `json:"localNodeName"`
	LocalRdmaInterface  string  `json:"localRdmaInterface"`
	LocalNetDev         string  `json:"localNetDev"`
	LocalPci            string  `json:"localPci"`
	LocalLinkSpeed      string  `json:"localLinkSpeed"`
	RemoteNodeName      string  `json:"remoteNodeName"`
	RemoteRdmaInterface string  `json:"remoteRdmaInterface"`
	RemoteNetDev        string  `json:"remoteNetDev"`
	RemotePci           string  `json:"remotePci"`
	RemoteLinkSpeed     string  `json:"remoteLinkSpeed"`
	TestResult          string  `json:"testResult"`
	ErrInfo             string  `json:"errInfo"`
	ErrInfoZh           string  `json:"errInfoZh"`
	BandWidth           float32 `json:"bandWidth"`
}
