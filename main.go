package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {

	file, err := os.Open("sample.txt")
	if err != nil {
		log.Fatal(err)
	}

	Scanner := bufio.NewScanner(file)
	Scanner.Split(bufio.ScanWords)

	for Scanner.Scan() {
		if strings.Contains(Scanner.Text(), "ng-version=") {
			fmt.Println("Gotcha")
			break
		}

		fmt.Println(Scanner.Text())
	}

	if err := Scanner.Err(); err != nil {
		log.Fatal(err)
	}
}
