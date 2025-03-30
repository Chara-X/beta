package builtin

func Copy[T any](dst []T, src []T) int {
	return copy(dst, src)
}
