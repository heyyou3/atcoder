package main

import "fmt"

func main() {
	var n, m int
	var sum int
	fmt.Scan(&n, &m)
	a := make([]int, m)

	for i := range a {
		fmt.Scan(&a[i])
		sum += a[i]
	}

	if n < sum {
		fmt.Println(-1)
	} else {
		fmt.Println(n - sum)
	}
}
