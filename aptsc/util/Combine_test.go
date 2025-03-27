package util

import "fmt"

func ExampleCombine() {
	// Example usage
	var arr1 = []any{1, 2}
	var arr2 = []any{"apple", "banana"}
	var arr3 = []any{true, false}
	// Combine the arrays into all possible combinations
	combinations := Combine(arr1, arr2, arr3)
	// Print the result
	fmt.Println("Combinations:")
	for _, comb := range combinations {
		fmt.Println(comb)
	}
	// Output:
	// Combinations:
	// [1 apple true]
	// [1 apple false]
	// [1 banana true]
	// [1 banana false]
	// [2 apple true]
	// [2 apple false]
	// [2 banana true]
	// [2 banana false]
}
