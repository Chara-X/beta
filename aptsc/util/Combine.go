package util

func Combine(arrs ...[]any) [][]any {
	// Base case: if no arrays, return an empty result
	if len(arrs) == 0 {
		return [][]any{}
	}
	// Base case: if only one array, wrap each element as a single combination
	if len(arrs) == 1 {
		var res [][]any
		for _, element := range arrs[0] {
			res = append(res, []any{element})
		}
		return res
	}
	// Recursive case: combine the first array with the combinations of the remaining arrays
	var firstArr = arrs[0]
	remainingCombination := Combine(arrs[1:]...)
	var res [][]any
	for _, element := range firstArr {
		for _, comb := range remainingCombination {
			// Prepend the current element to each combination from the recursive result
			res = append(res, append([]any{element}, comb...))
		}
	}
	return res
}
