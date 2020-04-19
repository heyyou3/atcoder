package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	a := make([]int, n-1)

	for i := range a {
		fmt.Scan(&a[i])
	}

	res := make([]int, n)

	for _, n := range a {
		res[n]++
	}

	for i, r := range res {
		if i == 0 {
			continue
		}
		fmt.Println(r)
	}
	fmt.Println(0)
}
