package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type gps struct {
	direction            string
	x                    int
	y                    int
	meta                 moveMeta
	visitedLocs          map[string]int
	firstLocVisitedTwice string
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
		direction:            "north",
		x:                    0,
		y:                    0,
		visitedLocs:          map[string]int{},
		firstLocVisitedTwice: "",
		meta: moveMeta{
			dirLookup: 0,
		},
	}

	for _, move := range moves {
		direction, numOfMoves := seperateDirectionAndNumber(move)

		gps.setDirBasedOnMove(direction)
		gps.moveNTimes(numOfMoves)
	}

	fmt.Println("the distance from the start is", calculateDistance(gps.x, gps.y), "spaces away")
	fmt.Println("(", gps.firstLocVisitedTwice, ") is the first location visited twice and is", calculateDistance(keyToCoords(gps.firstLocVisitedTwice)), "spaces away")
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
	for i := 0; i < n; i++ {
		if g.direction == "north" {
			g.addVisitedLocation()
			g.y++
		} else if g.direction == "south" {
			g.addVisitedLocation()
			g.y--
		} else if g.direction == "east" {
			g.addVisitedLocation()
			g.x++
		} else if g.direction == "west" {
			g.addVisitedLocation()
			g.x--
		}
	}
}

func (g *gps) addVisitedLocation() {
	locKey := fmt.Sprint(g.x, ",", g.y)

	g.visitedLocs[locKey]++

	if g.firstLocVisitedTwice == "" && g.visitedLocs[locKey] > 1 {
		g.firstLocVisitedTwice = locKey
	}
}

// get distance from start (this assumes start is at 0,0)
func calculateDistance(x, y int) int {
	return Abs(x) + Abs(y)
}

// absolute value function
func Abs(num int) int {
	if num < 0 {
		return -num
	}

	return num
}

// convert key to x,y coords
func keyToCoords(locKey string) (int, int) {
	arr := strings.Split(locKey, ",")
	x, _ := strconv.Atoi(arr[0])
	y, _ := strconv.Atoi(arr[1])
	return x, y
}
