import numpy as np
from collections import namedtuple

Position = namedtuple('Position', 'x y')

class Node(object):
  """A search node containing a board state"""

  def __init__(self, board, parent=None, blank=None):
    self.board = board
    self.size = board.shape[0]
    blank_pos = [x[0] for x in np.where(board == 0)]
    self.blank = blank if blank else Position(blank_pos[0], blank_pos[1])
    self.parent = parent
    self.moves = parent.moves + 1 if parent else 0

  def calculate_cost(self):
    goal_board = np.arange(1, self.size**2)
    goal_board.resize((self.size, self.size))

    self.cost = {
      'hamming': 0,
      'manhattan': 0,
      'linear_conflict': 0,
    }

    for i, row in enumerate(self.board):
      conflict_arr = []

      for j, val in enumerate(row):
        # Skipping the blank position
        if val == 0:
          continue

        goal_val = get_goal_value(i, j, self.size)
        goal_pos  = get_goal_position(val, self.size)
        # print(f'{val} > {i}-{goal_i} , {j}-{goal_j}')

        # Check if values mismatch by position
        self.cost['hamming'] += 1 if goal_val != val else 0
        # Calculate linear distance to goal position
        self.cost['manhattan'] += abs(goal_pos.x - i) + abs(goal_pos.y - j)

        if i == goal_pos.x:
          conflict_arr.append(val)

      for j, val in enumerate(conflict_arr):
        for next_val in conflict_arr[j+1:]:
          if val > next_val:
            self.cost['linear_conflict'] += 2
    
    self.cost['linear_conflict'] += self.cost['manhattan']

  def get_adjacent(self):
    adj_nodes = []

    for diff in (Position(0, 1), Position(0, -1), Position(1, 0), Position(-1, 0)):
      new_pos = Position(self.blank.x + diff.x, self.blank.y + diff.y)

      if not (0 <= new_pos.x < self.size) or not(0 <= new_pos.y < self.size):
        continue
      
      new_board = np.copy(self.board)
      new_board[self.blank.x][self.blank.y] = new_board[new_pos.x][new_pos.y]
      new_board[new_pos.x][new_pos.y] = 0

      adj_nodes.append(Node(new_board, self, new_pos))

    return adj_nodes

  def get_total_cost(self, name='hamming'):
    return self.moves + self.cost[name]

  def get_move(self):
    diff = self.blank.x - self.parent.blank.x, self.blank.y - self.parent.blank.y
    if diff == (1, 0):
      return "D"
    elif diff == (-1, 0):
      return "U"
    elif diff == (0, 1):
      return "R"
    elif diff == (0, -1):
      return "L"
    else:
      return "X"

  def __hash__(self):
    return hash((self.size, str(self.board)))

  def __eq__(self, obj):
    if isinstance(obj, np.ndarray):
      return np.array_equal(self.board, obj)

    return np.array_equal(self.board, obj.board)

  def __lt__(self, obj):
    # Secondary condition for pq when cost is equal
    # True: prioritizes newer nodes, faster/fewer exploration
    # False: more predicatable
    return False

  def __str__(self):
    msg = str(self.board)
    msg += f"\nMoves={self.moves}\n"
    msg += f"Hamming={self.cost['hamming']}, "\
           f"Manhattan={self.cost['manhattan']}, "\
           f"Conflict={self.cost['linear_conflict']}"
    return msg

def get_goal_value(i, j, size):
  """O(1)"""
  return i*size + j + 1

def get_goal_position(val, size):
  """O(1)"""
  return  Position((val - 1) // size, (val - 1) % size)
