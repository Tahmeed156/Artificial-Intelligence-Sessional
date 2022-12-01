import numpy as np
import heapq as Q
import time

from node import Node


def is_solvable(board):
  size = board.shape[0]
  blank = [x[0] for x in np.where(board == 0)]
  board_linear = board.reshape(-1)
  inversion_num = 0
  
  for i, x in enumerate(board_linear):
    for j, y in enumerate(board_linear[i+1:]):
      if y < x and y != 0:
        inversion_num += 1

  if (size % 2 == 0):
    if blank[0] % 2 == 0:
      return inversion_num % 2 == 1
    else:
      return inversion_num % 2 == 0
  else:
    return inversion_num % 2 == 0


def print_path(node):
  move_list = ""
  while node.parent is not None:
    #print(node.board)
    move_list += f"{node.get_move()}"
    node = node.parent

  print("blank moves: " + move_list[::-1])


def a_star_search(board, heuristic="hamming"):
  k = board.shape[0]
  final_board = np.arange(1, k**2)
  final_board.resize((k, k))

  explored = 0
  expanded = 0
  start_time = time.time_ns()

  start_node = Node(board)
  start_node.calculate_cost()

  open_pq = [] # Nodes to be explored
  closed_set = set() # Explored nodes
  Q.heappush(open_pq, (start_node.get_total_cost(heuristic), start_node))

  while open_pq:
    _, node = Q.heappop(open_pq)
    if node in closed_set:
      continue
    closed_set.add(node)
    expanded += 1

    # If node contains `final_board` state then return
    if np.array_equal(node.board, final_board):    
      print(f"> cost={node.moves} [{heuristic}]")
      print_path(node)
      return

    for adj_node in node.get_adjacent():
      if adj_node in closed_set:
        continue
      adj_node.calculate_cost()
      Q.heappush(open_pq, (adj_node.get_total_cost(heuristic), adj_node))
      explored += 1
      time_elapsed = (time.time_ns() - start_time) / (1e9)
      print(f"\r{explored=:7,}, {expanded=:7,}, {time_elapsed=:4.1f}s ", end="")
      # print(adj_node) # DEBUG

def array_input():
  try:
    with open('input.txt') as f:
      k = int(f.readline());
      input_board = np.zeros((k, k), dtype=int)
      
      for i, line in enumerate(f.readlines()):
        line = line.strip()
        val_arr = [x for x in line.split(' ') if x != ""]
        for j, val in enumerate(val_arr):
          if val == "*":
            val = 0
          input_board[i][j] = val
    
    print(input_board)
    return input_board
  except FileNotFoundError:
    print("Please give input in 'input.txt' file")
    exit(0)

if __name__ == "__main__":

  input_board = array_input()

  if not is_solvable(input_board):
    print("Puzzle is unsolvable!")
    exit(0)

  for h in ["hamming", "manhattan", "linear_conflict"]:
    a_star_search(input_board, h)
