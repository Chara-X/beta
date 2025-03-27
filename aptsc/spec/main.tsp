import "@typespec/http";
import "@typespec/rest";
import "@typespec/openapi3";
using TypeSpec.Http;
@service({
    title: "APTS OpenAPI Docs",
})
@server("https://10.166.209.110", "OpenPalette cluster-cim2")
@route("/opapi/wsm/v1/apts")
namespace APTS;

@tag("HealthCheck")
namespace HealthCheck {
    @post
    @route("/healthcheck")
    op create(@body _: Request): void;
    @get
    @route("/healthcheck")
    op get(): Response;
    model Request {
        object: string;
        checkSubInfo: Item;
        healthCheckCfg: Config;
    }
    model Item {
        gpuConsistencyCheck: GpuConsistencyCheck[];
        rdmaConsistencyCheck: RdmaConsistencyCheck[];
        gpuAvailabilityCheck: GpuAvailabilityCheck[];
        rdmaAvailabilityCheck: GpuAvailabilityCheck[];
        serverVerCheck: string[];
    }
    model Config {
        rdmaNums: uint32;
        gpuNums: uint32;
        paraNums?: uint32;
        storeNums?: uint32;
    }
    enum GpuConsistencyCheck {
        gpuharddropnum,
        gpusoftdropnum,
        gputopo,
        gpudriver,
        gpulinkspeed,
    }
    enum RdmaConsistencyCheck {
        rdmaportnum,
        rdmamtu,
        rdmaparaver,
        rdmastorever,
        rdmaresource,
    }
    enum GpuAvailabilityCheck {
        gpustatus,
        gpuprocessresidue,
        gpuresidue,
        gpudevice,
        gpuecc,
        gpupcie,
    }
    enum RdmaAvailabilityCheck {
        rdmaparalink,
        rdmastorelink,
        rdmaparaspeed,
        rdmastorespeed,
        rdmaparafibre,
        rdmastorefibre,
        rdmastatus,
        rdmaconfiguration,
        rdmapolicy,
    }
    model Response {
        result: Result[];
        status: string;
        startTime: string;
        endTime: string;
        vendor: string;
        nodeNum: uint32;
        gpuNum: uint32;
        paraNum?: uint32;
        storeNum?: uint32;
    }
    model Result {
        object: string;
        objectId: string[];
        checkType: string;
        checkSubItem: string;
        subResult: string;
        checkDetail: string;
        checkDetailEn: string;
        status: string;
        startTime: string;
        endTime: string;
        showDetail: boolean;
    }
}
@tag("CCLInspection")
namespace CCLInspection {
    @post
    @route("/scclinspection/start")
    op create(@body _: Request): void;
    @get
    @route("/scclinspection/query")
    op get(): Response;
    model Request {
        inspectType: string;
        cclType: string;
        command: string;
        gpuPerNode: uint32;
        nodeList: string[];
        testScene: string;
        outAlgbw: uint32;
        outBusbw: uint32;
        inAlgbw: uint32;
        inBusbw: uint32;
    }
    model Response {
        diag_created: string;
        diag_gpu_outputs: Output[];
        diag_total_count: uint32;
        inspect_type: string;
        status: string;
        task_result_summary: TaskResultSummary;
    }
    model Output {
        diag_exec_errmsg: string;
        diag_fail_count: uint32;
        gpuUUID: string;
        node_name: string;
        result: string;
        vendor: string;
    }
    model TaskResultSummary {
        totalCards: uint32;
        totalChecks: uint32;
        totalUsed: uint32;
    }
}
@tag("VCJobEvents")
namespace VCJobEvents {
    @get
    @route("/vcjobevents/{ns}/{name}")
    op get(@path("ns") ns: string, @path("name") name: string): Response;
    model Response {
        name: string;
        `namespace`: string;
        state: string;
        phase: string;
        creationTimestamp: string;
        duration: string;
        minAvailable: int32;
        running: int32;
        uid: string;
        conditions: Condition[];
        jobInfo: ResourceEventInfo[];
        podInfo: ResourceEventInfo[];
        gpuInfo: ResourceEventInfo[];
        rdmaDeviceInfo: ResourceEventInfo[];
        gpuTrainInfo: ResourceEventInfo[];
    }
    model Condition {
        type: string;
        status: string;
        lastProbeTime: string;
        lastTransitionTime: string;
        reason: string;
        message: string;
    }
    model ResourceEventInfo {
        name: string;
        `namespace`: string;
        type: string;
        message: string;
        time: string;
    }
}
@tag("NamespaceConfig")
namespace NamespaceConfig {
    @post
    @route("/namespace")
    op update(@body _: Request): void;
    model Request {
        nameSpace: string;
    }
    @get
    @route("/namespace")
    op get(): Response;
    model Response {
        nameSpace: string;
    }
}
@tag("ClRdmaInspectionConfig")
namespace ClRdmaInspectionConfig {
    @post
    @route("/clrdmainspectionconfig")
    op update(@body _: Request): void;
    model Request {
        clRdmaInspectionImageName: string;
    }
    @get
    @route("/clrdmainspectionconfig")
    op get(): Response;
    model Response {
        clRdmaInspectionImageName: string;
    }
}
@tag("ModelInspectionConfig")
namespace ModelInspectionConfig {
    @post
    @route("/modelinspectionconfig")
    op update(@body _: Request): void;
    model Request {
        codePath: string;
        firstStepTimeout: integer;
        generalShellPath: string;
        modelImageName: string;
        modelName: string;
        nonFirstStepTimeout: integer;
        perfOptShellPath: string;
    }
    @get
    @route("/modelinspectionconfig")
    op get(): Response;
    model Response {
        codePath: string;
        firstStepTimeout: integer;
        generalShellPath: string;
        modelImageName: string;
        modelName: string;
        nonFirstStepTimeout: integer;
        perfOptShellPath: string;
    }
}
@tag("RDMAInspection")
namespace RDMAInspection {
    @post
    @route("/rdma/submit")
    op create(@body _: Request): void;
    @get
    @route("/rdma/querybandwidth")
    op get(): Response;
    model Request {
        testScene: string;
        bandWidthThreshold: integer;
        nodeList: string[];
        rdmaPerNode: integer;
    }
    model Response {
        testRdmaResultList?: RDMAResult[];
        testScene?: string;
        status?: string;
        testRdmaResult?: string;
        errInfo?: string;
        errInfoZh?: string;
        testNodeNum?: integer;
        testRdmaNum?: integer;
        bandWidthThreshold: integer;
    }
    model RDMAResult {
        localNodeName: string;
        localRdmaInterface: string;
        localNetDev: string;
        localPci: string;
        localLinkSpeed: string;
        remoteNodeName: string;
        remoteRdmaInterface: string;
        remoteNetDev: string;
        remotePci: string;
        remoteLinkSpeed: string;
        testResult: string;
        errInfo: string;
        errInfoZh: string;
        bandWidth: float;
    }
}
@tag("ModelInspection")
namespace ModelInspection {
    @post
    @route("/modelinspection")
    op create(@body _: Request): void;
    @get
    @route("/modelinspection")
    op get(): Response;
    model Request {
        gpuTotal: integer;
        gpuPerNode: integer;
        taskId: string;
        nodeList: string[];
        superParam: string;
        cmd: string;
        cpuPerNode: integer;
        memoryPerNode: integer;
        gpuFactory: string;
        pvcEnable: string;
        pvcSize: string;
        pvcNamespace: string;
        pvcName: string;
        datasetName: string;
        modelImageName: string;
        modelImageTag: string;
        modelImagePullPolicy: string;
        modelImagePullSecrets: string[];
    }
    model Response {
        ModelTrainResults: AiModelTrainResultSpec[];
    }
    model AiModelTrainResultSpec {
        modelName: string;
        modelImageName: string;
        gpuFactory: string;
        gpuTotal: integer;
        cpuPerNode: integer;
        gpuPerNode: integer;
        memoryPerNode: integer;
        superParam: string;
        nodeNum: integer;
        exceResult: string;
        status: string;
        testResult: string;
        testResultZh: string;
        testScene: string;
        startTime: string;
        endTime: string;
        pvc: string;
        datasetName: string;
    }
}
