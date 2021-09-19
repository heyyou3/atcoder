package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

var sc = bufio.NewScanner(os.Stdin)

func main() {

	sc.Split(bufio.ScanWords)

	var n, inp int
	n = nextInt()
	var mn, ch int
	var sum int64
	for i := 0; i < n; i++ {
		inp = nextInt()
		mn += inp
		if ch < inp {
			ch = inp
		}
		sum += int64(inp * inp)
	}
	eu := math.Sqrt(float64(sum))
	fmt.Println(mn)
	fmt.Println(eu)
	fmt.Println(ch)
}

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	if i < 0 {
		i *= -1
	}
	return i
}
