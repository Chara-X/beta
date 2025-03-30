package builtin

func NewChannel[T any](capacity int) chan T { return make(chan T, capacity) }

type Channel[T any] chan T

func (channel *Channel[T]) Close() { close(*channel) }
