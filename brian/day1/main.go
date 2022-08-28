package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type location struct {
	x                int
	y                int
	currentDirection string
}

func main() {
	playerLoc := location{
		x:                0,
		y:                0,
		currentDirection: "north",
	}

	inputs := readFile("inputs.txt")

	for _, move := range inputs {
		playerLoc.changeDirection(move)
	}

	fmt.Println(getDistance(playerLoc))
}

func (loc *location) changeDirection(move string) {
	moveDirection, moveAmount := string(move[1]), string(move[2:])
	convMoveAmount, _ := strconv.Atoi(moveAmount)
	fmt.Println("direction", moveDirection, len(moveDirection))

	if loc.currentDirection == "north" {
		if moveDirection == "R" {
			loc.currentDirection = "east"
			loc.x += convMoveAmount
		} else {
			loc.currentDirection = "west"
			loc.x -= convMoveAmount
		}
	} else if loc.currentDirection == "south" {
		if moveDirection == "R" {
			loc.currentDirection = "west"
			loc.x -= convMoveAmount
		} else {
			loc.currentDirection = "east"
			loc.x += convMoveAmount
		}
	} else if loc.currentDirection == "east" {
		if moveDirection == "R" {
			loc.currentDirection = "south"
			loc.y -= convMoveAmount
		} else {
			loc.currentDirection = "north"
			loc.y += convMoveAmount
		}
	} else if loc.currentDirection == "west" {
		if moveDirection == "R" {
			loc.currentDirection = "north"
			loc.y += convMoveAmount
		} else {
			loc.currentDirection = "south"
			loc.y -= convMoveAmount
		}
	}
	fmt.Println(*loc)
}

func getDistance(loc location) int {
	return (int(math.Abs(float64(loc.x)))) + int(math.Abs(float64(loc.y)))
}
func readFile(fileName string) []string {
	bs, _ := os.ReadFile(fileName)
	return strings.Split(string(bs), ",")
}
