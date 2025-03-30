package builtin

func Append[T any](slice []T, elems ...T) []T {
	return append(slice, elems...)
}
