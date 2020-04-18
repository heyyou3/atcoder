package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	var res int
	for i := 1; i <= n; i++ {
		if i%3 == 0 || i%5 == 0 {
			continue
		}
		res += i
	}
	fmt.Println(res)
}
