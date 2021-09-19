package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var (
	sc = bufio.NewScanner(os.Stdin)
)

func nextLine(scanner *bufio.Scanner) string {
	scanner.Scan()
	return scanner.Text()
}

func nextInts(scanner *bufio.Scanner) (res []int) {
	scanner.Scan()
	s := scanner.Text()
	for _, v := range strings.Split(s, " ") {
		i, _ := strconv.Atoi(v)
		res = append(res, i)
	}
	return
}

func resolver(n, a, b int) int {
	return (n + b) - a
}

func main() {
	inputs := nextInts(sc)
	n, a, b := inputs[0], inputs[1], inputs[2]
	fmt.Println(resolver(n, a, b))
}
