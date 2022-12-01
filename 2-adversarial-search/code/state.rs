use crate::GameResult;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct State {
  pub board: [u8; 14],
  pub range: Vec<u8>,
  pub store: u8,
  pub opp_range: Vec<u8>,
  pub opp_store: u8,
  pub first_index: u8,
  pub last_index: u8,
}

impl State {
  pub fn initial() -> Self {
    Self {
      board: [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4],
      range: vec![1, 2, 3, 4, 5, 6],
      opp_range: vec![8, 9, 10, 11, 12, 13],
      store: 7,
      opp_store: 0,
      first_index: 0,
      last_index: 0,
    }
  }

  pub fn is_terminal(&self) -> bool {
    let p1_bin_sum: u8 = self.board[1..=6].iter().sum();
    let p2_bin_sum: u8 = self.board[8..=13].iter().sum();
    p1_bin_sum == 0 || p2_bin_sum == 0
  }

  pub fn get_moves(&self) -> Vec<u8> {
    let mut bins: Vec<u8> = Vec::new();
    for &bin in self.range.iter() {
      if self.board[bin as usize] != 0 { 
        bins.push(bin);
      }
    }
    let mut rng = thread_rng();
    bins.shuffle(&mut rng);
    bins
  }

  pub fn swap_turn(&mut self) {
    std::mem::swap(&mut self.store, &mut self.opp_store);
    std::mem::swap(&mut self.range, &mut self.opp_range);
  }

  pub fn make_move(&self, bin: u8) -> Self {
    let mut board: [u8; 14] = self.board.clone();
    let stones = board[bin as usize];
    board[bin as usize] = 0;

    let first_index = (bin + 1) % 14;
    let mut last_index = (bin + stones) % 14;

    // Adding stones to next bins
    let mut index = first_index;
    while index != (last_index + 1) % 14 {
      // Skip if opponent store
      if index == self.opp_store {
        last_index = (last_index + 1) % 14;
      } else {
        board[index as usize] += 1;
      }
      index = (index + 1) % 14;
    }

    State {
      first_index: first_index,
      last_index: last_index,
      board: board,
      range: self.range.clone(), // TODO: References should've been enough as this is unchanged
      opp_range: self.opp_range.clone(),
      ..*self
    }
  }

  pub fn is_bonus_turn(&self) -> bool {
    self.last_index == self.store
  }

  pub fn can_capture(&self) -> bool {
    // If last stone drops on empty bin on player's side
    // Skip if opponent's bin is empty
    let opp_index: u8 = self.store + (7 - (self.last_index % 7));
    self.board[self.last_index as usize] == 1 && 
      self.range.contains(&self.last_index) && 
      self.board[opp_index as usize] != 0
  }

  pub fn perform_capture(&mut self) {
    let opp_index: u8 = self.store + (7 - (self.last_index % 7));
    self.board[self.store as usize] += self.board[self.last_index as usize] + self.board[opp_index as usize];
    self.board[self.last_index as usize] = 0;
    self.board[opp_index as usize] = 0;
  }

  pub fn collect_remaining(&mut self) {
    // Collect all bins from player 1 to their store
    let p1_bin_sum: u8 = self.board[1..=6].iter().sum();
    self.board[7] += p1_bin_sum;
    for bin in 1..=6 {
      self.board[bin] = 0;
    }
    // Collect all bins from player 2 to their store
    let p2_bin_sum: u8 = self.board[8..=13].iter().sum();
    self.board[0] += p2_bin_sum;
    for bin in 8..=13 {
      self.board[bin] = 0;
    }
  }

  pub fn get_results(&self) -> GameResult {
    return match self.board[7] {
      d if d > self.board[0] => GameResult(1, self.board[7], self.board[0]),
      d if d < self.board[0] => GameResult(-1, self.board[7], self.board[0]),
      _ => GameResult(0, self.board[7], self.board[0]),
    }
  }
}