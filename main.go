package main

import (
	"fmt"
	"os"
)

func readFromFile(path string) (string, error) {
	source, err := os.ReadFile(path)

	if err != nil {
		return "", err
	}

	return string(source), nil
}

func run() {
	args := os.Args

	if len(args) == 2 {
		path := args[1]
		source, err := readFromFile(path)

		if err != nil {
			panic(err)
		}

		fmt.Println(source)

	} else {
		panic("Implement REPL")
	}
}

func main() {
	run()
}
