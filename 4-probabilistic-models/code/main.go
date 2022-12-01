package main

import (
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("./input.txt")
	if err != nil {
		panic("Couldn't open file")
	}
	defer f.Close()

	// Create board [n x m]
	var n, m, k int
	fmt.Fscanf(f, "%d %d %d\n", &n, &m, &k)
	game := Game{}
	game.initializeBoard(n, m, k)

	// Setting obstacles
	var u, v int
	for i := 0; i < k; i++ {
		fmt.Fscanf(f, "%d %d\n", &u, &v)
		game.board[u][v].setObstacle()
	}

	game.calcAdjc()

	var opt byte
	for {
		n, err := fmt.Fscanf(f, "%c", &opt)
		if n == 0 || err != nil {
			break
		}

		switch opt {
		case 'R':
			fmt.Fscanf(f, " %d %d %d\n", &n, &m, &k)
			fmt.Println("$ Observed position (", n, m, ") and detected = ", k)
			game.timeStep()
			game.observe(n, m, k)
		case 'C':
			fmt.Fscanf(f, "\n")
			n, m = game.getCasper()
			fmt.Println("$ Probable position: (", n, m, ")")
		case 'Q':
			fmt.Println("$ Say goodbye to Casper :(")
			return
		}
	}
}
