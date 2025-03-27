package assert

import (
	"errors"

	"github.com/Chara-X/aptsc/msg"
)

func AfterCCLInspectionFinish(res *msg.CCLInspectionGetResponse) {
	if res.Status == "failed" {
		panic(errors.New("CCLInspectionGet failed"))
	}
}
