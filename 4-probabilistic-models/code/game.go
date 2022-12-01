package main

type Game struct {
	board      Board
	globalTime uint
}

func (game Game) getCasper() (x, y int) {
	var maxProb float64 = -1

	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			if game.board[i][j].belief > maxProb {
				maxProb = game.board[i][j].belief
				x, y = i, j
			}
		}
	}
	return
}

func (game Game) observe(x, y, detc int) {
	pos := Position{x, y}

	// Positions affected by sensor range
	affPos := map[Position]byte{
		{pos.x, pos.y}: 1,
		// Corner
		{pos.x + 1, pos.y + 1}: 1,
		{pos.x - 1, pos.y + 1}: 1,
		{pos.x + 1, pos.y - 1}: 1,
		{pos.x - 1, pos.y - 1}: 1,
		// Edge
		{pos.x + 1, pos.y}: 1,
		{pos.x - 1, pos.y}: 1,
		{pos.x, pos.y + 1}: 1,
		{pos.x, pos.y - 1}: 1,
	}

	// Probabilities of affected cells & others
	affProb, otherProb := func() (float64, float64) {
		if detc == 1 {
			return 0.85, 0.15
		} else {
			return 0.15, 0.85
		}
	}()

	// Updating probabilities
	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			cell := Position{i, j}
			_, exists := affPos[cell]
			if exists {
				game.board[i][j].belief = affProb * game.board[i][j].belief
			} else {
				game.board[i][j].belief = otherProb * game.board[i][j].belief
			}
		}
	}

	game.board.normalize()

	game.board.print()
}

func (game *Game) initializeBoard(n, m, k int) {
	game.board = makeBoard(n, m)

	// Initialize probability
	var initProb float64 = 1. / ((float64(n) * float64(m)) - float64(k))
	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			cell := &game.board[i][j]
			cell.belief = initProb
			cell.pos = Position{i, j}
		}
	}
}

func (game *Game) calcAdjc() {

	// Setting edge, corner info
	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			cell := &game.board[i][j]
			cell.numEdgeCells = game.board.getEdgeCellNum(cell.pos)
			cell.numCornerCells = game.board.getCornerCellNum(cell.pos)
		}
	}

	game.board.print()
}

func (game *Game) timeStep() {

	// Set previous belief to current, current to 0
	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			game.board[i][j].prevBelief = game.board[i][j].belief
			game.board[i][j].belief = 0
		}
	}

	// For each empty cell calculate new probability
	for i := 0; i < game.board.getN(); i++ {
		for j := 0; j < game.board.getM(); j++ {
			// Skip for obstacle
			if game.board[i][j].isObstacle {
				continue
			}
			cell := &game.board[i][j]
			totalEdgeProb, totalOtherProb := func() (float64, float64) {
				if cell.numEdgeCells > 0 {
					return 0.9, 0.1
				} else {
					return 0.0, 1.0
				}
			}()
			// Propagating probability to edge cells
			edgeProb := totalEdgeProb / cell.numEdgeCells
			game.board.addEdgeCellProb(cell.pos, cell.prevBelief*edgeProb)
			// Propagating probability to corner cells
			otherProb := totalOtherProb / (cell.numCornerCells + 1)
			game.board.addCornerCellProb(cell.pos, cell.prevBelief*otherProb)
			// Adding probability of staying in position
			cell.belief += cell.prevBelief * otherProb
		}
	}

	game.globalTime += 1
}
