package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Action int

const (
	End Action = iota
	Walk
	Run
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Println("1: Walk, 2: Run, 0: End")
		fmt.Print("> ")

		if !scanner.Scan() {
			fmt.Println("Failed to read input.")
			break
		}

		input := scanner.Text()
		action, err := strconv.Atoi(input)

		if err != nil {
			fmt.Println("Invalid input")
			continue
		}

		switch Action(action) {
		case Walk:
			fmt.Println("Walking")
		case Run:
			fmt.Println("Running")
		case End:
			return
		default:
			fmt.Println("Invalid input")
		}
	}
}

