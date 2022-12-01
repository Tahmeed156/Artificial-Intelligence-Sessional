package main

import "fmt"

type Cell struct {
	belief         float64
	prevBelief     float64
	pos            Position
	numEdgeCells   float64
	numCornerCells float64
	isObstacle     bool
}

type Position struct {
	x, y int
}

type Board [][]Cell

func makeBoard(n, m int) Board {
	board := make([][]Cell, n)
	for i := 0; i < n; i++ {
		board[i] = make([]Cell, m)
	}
	return board
}

func (board Board) isEmptyCell(pos Position) bool {
	// x coordinate out of board range
	if pos.x >= board.getN() || pos.x < 0 {
		return false
	}
	// y coordinate out of board range
	if pos.y >= board.getM() || pos.y < 0 {
		return false
	}
	// Cell is an obstacle
	if board[pos.x][pos.y].isObstacle {
		return false
	}
	return true
}

func (board Board) getEdgeCellNum(pos Position) float64 {
	newPos := []Position{
		{pos.x + 1, pos.y},
		{pos.x - 1, pos.y},
		{pos.x, pos.y + 1},
		{pos.x, pos.y - 1},
	}
	count := 0

	for i := 0; i < len(newPos); i++ {
		if board.isEmptyCell(newPos[i]) {
			count = count + 1
		}
	}
	return float64(count)
}

func (board Board) getCornerCellNum(pos Position) float64 {
	newPos := []Position{
		{pos.x + 1, pos.y + 1},
		{pos.x - 1, pos.y + 1},
		{pos.x + 1, pos.y - 1},
		{pos.x - 1, pos.y - 1},
	}
	count := 0

	for i := 0; i < len(newPos); i++ {
		if board.isEmptyCell(newPos[i]) {
			count = count + 1
		}
	}
	return float64(count)
}

func (board Board) print() {
	for i := 0; i < board.getN(); i++ {
		for j := 0; j < board.getM(); j++ {
			fmt.Printf("%.6f\t", board[i][j].belief)
		}
		fmt.Println()
	}
}

func (board Board) addEdgeCellProb(pos Position, prob float64) {
	newPos := []Position{
		{pos.x + 1, pos.y},
		{pos.x - 1, pos.y},
		{pos.x, pos.y + 1},
		{pos.x, pos.y - 1},
	}

	for i := 0; i < len(newPos); i++ {
		if board.isEmptyCell(newPos[i]) {
			board[newPos[i].x][newPos[i].y].belief += prob
		}
	}
}

func (board Board) addCornerCellProb(pos Position, prob float64) {
	newPos := []Position{
		{pos.x + 1, pos.y + 1},
		{pos.x - 1, pos.y + 1},
		{pos.x + 1, pos.y - 1},
		{pos.x - 1, pos.y - 1},
	}

	for i := 0; i < len(newPos); i++ {
		if board.isEmptyCell(newPos[i]) {
			board[newPos[i].x][newPos[i].y].belief += prob
		}
	}
}

func (board Board) normalize() {

	var totalProb float64 = 0
	for i := 0; i < board.getN(); i++ {
		for j := 0; j < board.getM(); j++ {
			totalProb += board[i][j].belief
		}
	}

	for i := 0; i < board.getN(); i++ {
		for j := 0; j < board.getM(); j++ {
			board[i][j].belief /= totalProb
		}
	}
}

func (board Board) getN() int {
	return len(board)
}

func (board Board) getM() int {
	return len(board[0])
}

func (cell *Cell) setObstacle() {
	cell.belief = 0
	cell.isObstacle = true
}
