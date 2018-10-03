package main

import (
	"fmt"
)

// WasRun ...
type WasRun struct {
	name string
	wasRun int
}

func wasRun(name string) WasRun {
	return WasRun{
		name: name,
		wasRun: 0,
	}
}

func (r *WasRun) run() {
	r.testMethod()
}

func (r *WasRun) testMethod() {
	r.wasRun = 1
}

func main() {
	test := wasRun("testMethod")
	fmt.Println(test.wasRun)
	test.run()
	fmt.Println(test.wasRun)
}
