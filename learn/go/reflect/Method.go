package reflect

import "reflect"

type Method struct {
	Name    string
	PkgPath string
	Type    Type
	Index   int
	Func    reflect.Value
}

func (m Method) IsExported() bool
