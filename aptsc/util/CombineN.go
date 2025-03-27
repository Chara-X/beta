package util

func CombineN[T any](arr []T, n int) [][]T {
	var res [][]T
	var combine func(start int, comb []T)
	combine = func(start int, comb []T) {
		if len(comb) == n {
			res = append(res, append([]T(nil), comb...))
			return
		}
		for i := start; i < len(arr); i++ {
			combine(i+1, append(comb, arr[i]))
		}
	}
	combine(0, []T{})
	return res
}
