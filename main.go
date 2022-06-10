package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	stringToDelete := "ng-version=\""

	file, err := os.OpenFile("sample.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)

	if err != nil {
		log.Fatal(err)
	}

	Scanner := bufio.NewScanner(file)
	Scanner.Split(bufio.ScanWords)

	for Scanner.Scan() {
		if strings.Contains(Scanner.Text(), stringToDelete) {
			items := strings.Split(Scanner.Text(), " ")
			fmt.Println(items)
			for _, item := range items {
				fmt.Println(item)
				if strings.Contains(item, stringToDelete) {
					item = ">"
					fmt.Println(item)
					file.WriteString(item);
				}
			}
			fmt.Println("Gotcha")
			break
		}

		fmt.Println(Scanner.Text())
	}

	if err := Scanner.Err(); err != nil {
		log.Fatal(err)
	}

	file.Close()
}
