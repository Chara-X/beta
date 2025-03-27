package global

func NewSlice[T any](length, capacity int) []T { return make([]T, length, capacity) }

type Slice[T any] []T

func (slice *Slice[T]) Append(elements ...T) Slice[T] { return append(*slice, elements...) }
func (slice *Slice[T]) Copy(source Slice[T]) int      { return copy(*slice, source) }
func (slice *Slice[T]) Clear()                        { clear(*slice) }
