package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	moves := fileToCode("inputs.txt")

	for _, move := range moves {
		direction, numOfMoves := seperateDirectionAndNumber(move)

		fmt.Println(direction, numOfMoves)
	}
}

func fileToCode(fileName string) []string {
	bs, _ := os.ReadFile(fileName)

	return strings.Split(string(bs), ", ")
}

func seperateDirectionAndNumber(dirAndNum string) (string, int) {
	dir, numMoves := string(dirAndNum[0]), dirAndNum[1:]
	convertedNumMoves, _ := strconv.Atoi(numMoves)

	return dir, convertedNumMoves
}
