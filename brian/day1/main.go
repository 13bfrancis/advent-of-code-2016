package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type gps struct {
	direction string
	x         int
	y         int
	meta      moveMeta
}

type moveMeta struct {
	dirLookup int
}

var directionMap = map[int]string{
	0: "north",
	1: "east",
	2: "south",
	3: "west",
}

func main() {
	moves := fileToCode("inputs.txt")
	gps := gps{
		direction: "north",
		x:         0,
		y:         0,
		meta: moveMeta{
			dirLookup: 0,
		},
	}

	for _, move := range moves {
		direction, numOfMoves := seperateDirectionAndNumber(move)

		gps.setDirBasedOnMove(direction)
		gps.moveNTimes(numOfMoves)
	}

	fmt.Println(gps.calculateDistance())
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

func (g *gps) setDirBasedOnMove(moveDir string) {
	if moveDir == "L" {
		g.meta.dirLookup--
	} else if moveDir == "R" {
		g.meta.dirLookup++
	}

	if g.meta.dirLookup == -1 {
		g.meta.dirLookup = 3
	}
	if g.meta.dirLookup == 4 {
		g.meta.dirLookup = 0
	}

	g.direction = directionMap[g.meta.dirLookup]
}

func (g *gps) moveNTimes(n int) {
	if g.direction == "north" {
		g.y += n
	} else if g.direction == "south" {
		g.y -= n
	} else if g.direction == "east" {
		g.x += n
	} else if g.direction == "west" {
		g.x -= n
	}
}

func (g *gps) calculateDistance() int {
	return Abs(g.x) + Abs(g.y)
}

// absolute value function
func Abs(num int) int {
	if num < 0 {
		return -num
	}

	return num
}
