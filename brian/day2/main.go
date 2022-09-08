package main

import (
	"fmt"
	"os"
	"strings"
)

type keypad struct {
	keys             [][]string
	lastPressedIndex location
	currentLocation  location
}

type location struct {
	row int
	col int
}

func main() {
	kp := keypad{
		keys: [][]string{
			{"1", "2", "3"},
			{"4", "5", "6"},
			{"7", "8", "9"},
		},
		currentLocation:  location{1, 1},
		lastPressedIndex: location{1, 1},
	}

	fmt.Println(kp)
}

func (kp *keypad) moveUp() {
	if kp.isValidMove(-1, 0) {
		kp.setNewLocation(kp.currentLocation.row-1, kp.currentLocation.col)
	}
}

func (kp *keypad) moveDown() {
	if kp.isValidMove(1, 0) {
		kp.setNewLocation(kp.currentLocation.row+1, kp.currentLocation.col)
	}
}

func (kp *keypad) moveLeft() {
	if kp.isValidMove(0, -1) {
		kp.setNewLocation(kp.currentLocation.row, kp.currentLocation.col-1)
	}
}

func (kp *keypad) moveRight() {
	if kp.isValidMove(0, 1) {
		kp.setNewLocation(kp.currentLocation.row, kp.currentLocation.col+1)
	}
}

func (kp *keypad) setNewLocation(row int, col int) {
	kp.currentLocation = location{row, col}
}

func (kp *keypad) isValidMove(row int, col int) bool {
	newRowIndex := row + kp.currentLocation.row
	newColIndex := col + kp.currentLocation.col

	if newRowIndex < 0 || newColIndex < 0 {
		return false
	}

	if newRowIndex > len(kp.keys)-1 {
		return false
	}

	if newColIndex > len(kp.keys[kp.currentLocation.row])-1 {
		return false
	}

	return true
}

func convertInputsToCode() []string {
	bs, _ := os.ReadFile("inputs.txt")
	lines := strings.Split(string(bs), "\n")

	return lines
}
