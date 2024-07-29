package main

import (
	"flag"
	"fmt"
)

func main() {
	url := flag.String("url", "", "")
	flag.Parse()
	fmt.Println("Hello! Gdownd!")
	fmt.Println(*url)
}
