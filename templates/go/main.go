package main

import (
	"bufio"
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

func resolver() int {
	return 0
}

func main() {
}
