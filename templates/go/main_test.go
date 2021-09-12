package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var (
	PROB = "a"
)

type sampleTestCase struct {
	out string
}

func (stc *sampleTestCase) OutputFileName(sampleFileName string) string {
	wd, _ := os.Getwd()
	return fmt.Sprintf("%s/tests/%s.out", wd, sampleFileName)
}

func (stc *sampleTestCase) IntOutput() int {
	outI, _ := strconv.Atoi(stc.out)
	return outI
}

func NewSampleCase(sampleFileName string) *sampleTestCase {
	stc := &sampleTestCase{}
	outFile, _ := os.Open(stc.OutputFileName(sampleFileName))
	outSc := bufio.NewScanner(outFile)
	outSc.Scan()
	stc.out = outSc.Text()
	return stc
}
