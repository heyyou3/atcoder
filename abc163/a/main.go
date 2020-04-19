package main

import (
	"fmt"
	"math"
)

func main() {
	var n int
	fmt.Scan(&n)
	fmt.Println(math.Pi * float64((n * 2)))
}
