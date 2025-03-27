package gomonkey

type Patches struct{}

func NewPatches() *Patches
func (this *Patches) ApplyFunc(target, double interface{}) *Patches
func (this *Patches) ApplyMethod(target interface{}, methodName string, double interface{}) *Patches
func (this *Patches) Origin(fn func())
func (this *Patches) Reset()
