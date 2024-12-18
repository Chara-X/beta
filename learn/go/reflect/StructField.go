package reflect

import "reflect"

type StructField struct {
	Name      string
	PkgPath   string
	Type      Type
	Index     []int
	Tag       reflect.StructTag
	Anonymous bool
}

func (field StructField) IsExported() bool
