package main

import (
	"fmt"
	"os"
	"strings"
)

type keypad struct {
	keys            [][]string
	currentLocation location
}

type location struct {
	row int
	col int
}

func main() {
	kp1 := keypad{
		keys: [][]string{
			{"1", "2", "3"},
			{"4", "5", "6"},
			{"7", "8", "9"},
		},
		currentLocation: location{1, 1},
	}

	kp2 := keypad{
		keys: [][]string{
			{"_", "_", "1", "_", "_"},
			{"_", "2", "3", "4", "_"},
			{"5", "6", "7", "8", "9"},
			{"_", "A", "B", "C", "_"},
			{"_", "_", "D", "_", "_"},
		},
		currentLocation: location{2, 0},
	}

	moves := convertInputsToCode()
	fmt.Println("part 1")
	navigateBoard(kp1, moves)
	fmt.Println("part 2")
	navigateBoard(kp2, moves)
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

func (kp *keypad) pressButton(row int, col int) {
	fmt.Println(kp.keys[row][col])
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

	return kp.keys[newRowIndex][newColIndex] != "_"
}

func navigateBoard(kp keypad, moves []string) {
	moveMap := map[string]func(){
		"U": kp.moveUp,
		"D": kp.moveDown,
		"L": kp.moveLeft,
		"R": kp.moveRight,
	}

	for _, move := range moves {
		for _, direction := range move {
			moveMap[string(direction)]()
		}
		kp.pressButton(kp.currentLocation.row, kp.currentLocation.col)
	}
}

func convertInputsToCode() []string {
	bs, _ := os.ReadFile("inputs.txt")
	lines := strings.Split(string(bs), "\n")

	return lines
}
