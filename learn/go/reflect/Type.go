package reflect

import "reflect"

type Type interface {
	Name() string
	PkgPath() string
	Kind() reflect.Kind
	NumField() int
	Field(index int) StructField
	NumMethod() int
	Method(index int) Method
	NumIn() int
	In(index int) Type
	NumOut() int
	Out(index int) Type
	IsVariadic() bool
	Len() int
	Key() Type
	Elem() Type
	ChanDir() reflect.ChanDir
	Implements(targetType Type) bool
	AssignableTo(targetType Type) bool
	ConvertibleTo(targetType Type) bool
}

func TypeOf(value any) Type
func TypeFor[T any]() Type { panic(0) }
func StructOf(fields []StructField) Type
func FuncOf(in, out []Type, isVariadic bool) Type
func ArrayOf(length int, elementType Type) Type
func SliceOf(elementType Type) Type
func MapOf(keyType, elementType Type) Type
func ChanOf(direction reflect.ChanDir, elementType Type) Type
func PointerTo(targetType Type) Type

/*
Value: dynamic
*/
