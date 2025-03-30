package atomic

import "sync/atomic"

var _ atomic.Value

// [atomic.Pointer]
type Pointer[T any] struct{}

// [atomic.Pointer.Load]
func (x *Pointer[T]) Load() *T

// [atomic.Pointer.Store]
func (x *Pointer[T]) Store(val *T)

// [atomic.Pointer.Swap]
func (x *Pointer[T]) Swap(new *T) (old *T)

// [atomic.Pointer.CompareAndSwap]
func (x *Pointer[T]) CompareAndSwap(old, new *T) (swapped bool)
