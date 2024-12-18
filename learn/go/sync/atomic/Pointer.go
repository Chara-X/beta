package atomic

type Pointer[T any] struct{}

func (x *Pointer[T]) Load() *T
func (x *Pointer[T]) Store(val *T)
func (x *Pointer[T]) Swap(new *T) (old *T)
func (x *Pointer[T]) CompareAndSwap(old, new *T) (swapped bool)
