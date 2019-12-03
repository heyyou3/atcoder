package main

import (
	"fmt"
	"strings"
)

func main() {
	var intNum int
	fmt.Scanf("%d", &intNum)
	var textArr string
	fmt.Scanf("%s", &textArr)
	var arrA []string
	for _, a := range textArr {
		total := int(a) + intNum
		var tmpInt int
		switch {
		case total > 90:
			diff := total - 90
			if diff > 26 {
				surplus := diff % 26
				tmpInt = 64 + surplus
			} else {
				tmpInt = 64 + diff
			}
		default:
			tmpInt = total
		}
		arrA = append(arrA, string(tmpInt))
	}
	output := strings.Join(arrA, "")
	fmt.Println(output)
}
