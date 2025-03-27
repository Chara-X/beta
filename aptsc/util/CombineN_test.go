package util

import (
	"fmt"
)

func ExampleCombineN() {
	arr := []int{1, 2, 3, 4}
	n := 3
	combinations := CombineN(arr, n)
	// Print all combinations
	fmt.Println("Combinations:")
	for _, comb := range combinations {
		fmt.Println(comb)
	}
	// Output:
	// Combinations:
	// [1 2 3]
	// [1 2 4]
	// [1 3 4]
	// [2 3 4]
}
