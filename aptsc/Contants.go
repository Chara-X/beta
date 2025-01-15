package aptsc

const (
	NamespaceConfigUpdatePath = "/opapi/wsm/v1/apts/namespace"
	NamespaceConfigGetPath    = "/opapi/wsm/v1/apts/namespace"
)
const (
	GPUInspectionCreatePath = "/opapi/wsm/v1/apts/gpuinspectiontask"
	GPUInspectionGetPath    = "/opapi/wsm/v1/apts/gpuinspectiontask"
	GPUInspectionDeletePath = "/opapi/wsm/v1/apts/gpuinspectiontask"
)
const (
	CCLInspectionCreatePath = "/opapi/wsm/v1/apts/scclinspection/start"
	CCLInspectionGetPath    = "/opapi/wsm/v1/apts/scclinspection/query"
	CCLInspectionDeletePath = "/opapi/wsm/v1/apts/scclinspection/stop"
)
const (
	HealthCheckCreatePath = "/opapi/wsm/v1/apts/healthcheck"
	HealthCheckGetPath    = "/opapi/wsm/v1/apts/healthcheck"
)
const (
	DatasetCreatePath = "/opapi/wsm/v1/apts/dataset"
	DatasetBindPath   = "/opapi/wsm/v1/apts/dataset/bind"
	DatasetUnbindPath = "/opapi/wsm/v1/apts/dataset/unbind/%v" // @prompt name
	DatasetDeletePath = "/opapi/wsm/v1/apts/dataset/%v"        // @prompt name
	DatasetGetPath    = "/opapi/wsm/v1/apts/datasets"
)
