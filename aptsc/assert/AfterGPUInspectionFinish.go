package assert

import (
	"errors"

	"github.com/Chara-X/aptsc/msg"
)

func AfterGPUInspectionFinish(res *msg.GPUInspectionGetResponse) {
	if len(res.Diag_gpu_outputs) != res.Task_result_summary.FailChecks+res.Task_result_summary.TotalUsed {
		panic(errors.New("GPUInspection line count mismatch summary"))
	}
}
