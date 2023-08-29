package main

import (
	"fmt"
)

func main() {
	fmt.Printf("isPalindrome(): %v\n", isPalindrome(121))
}

func isPalindrome(a int) bool {
	if a < 0 {
		return false
	}

	sum := 0
	original := a

	for a != 0 {
		r := a % 10
		sum = sum*10 + r
		a = a / 10
	}

	return sum == original
}
