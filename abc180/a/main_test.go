package main

import (
	"bufio"
	"fmt"
	"os"
	"reflect"
	"strconv"
	"testing"
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

func Test_nextLine(t *testing.T) {
	type args struct {
		scanner *bufio.Scanner
	}
	tests := []struct {
		name string
		args args
		want string
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := nextLine(tt.args.scanner); got != tt.want {
				t.Errorf("nextLine() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_nextInts(t *testing.T) {
	type args struct {
		scanner *bufio.Scanner
	}
	tests := []struct {
		name    string
		args    args
		wantRes []int
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if gotRes := nextInts(tt.args.scanner); !reflect.DeepEqual(gotRes, tt.wantRes) {
				t.Errorf("nextInts() = %v, want %v", gotRes, tt.wantRes)
			}
		})
	}
}

func Test_main(t *testing.T) {
	tests := []struct {
		name string
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			main()
		})
	}
}

func Test_resolver(t *testing.T) {
	type args struct {
		n int
		a int
		b int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{args: args{100, 1, 2}, want: 101},
		{args: args{100, 2, 1}, want: 99},
		{args: args{100, 1, 1}, want: 100},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := resolver(tt.args.n, tt.args.a, tt.args.b); got != tt.want {
				t.Errorf("resolver() = %v, want %v", got, tt.want)
			}
		})
	}
}
