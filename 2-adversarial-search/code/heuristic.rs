pub struct Heuristic {
  h_type: u8,
  store: u8,
  opp_store: u8,
  range: Vec<u8>,
  opp_range: Vec<u8>,
}

impl Heuristic {
  pub fn new(is_p1: bool, h_type: u8) -> Heuristic {
    match is_p1 {
      true => Heuristic {
        h_type: h_type,
        store: 7,
        opp_store: 0,
        range: vec![1, 2, 3, 4, 5, 6],
        opp_range: vec![8, 9, 10, 11, 12, 13],
      },
      false => Heuristic {
        h_type: h_type,
        store: 0,
        opp_store: 7,
        range: vec![8, 9, 10, 11, 12, 13],
        opp_range: vec![1, 2, 3, 4, 5, 6],
      }
    }
  }

  pub fn get_value(&self, board: &[u8; 14]) -> f32 {
    match self.h_type {
      1 => self.store_diff(board),
      2 => { 
        0.6 * self.store_diff(board) +
        0.4 * self.bin_stone_diff(board)
      },
      3 => { 
        0.2 * self.store_diff(board) +
        0.3 * self.bin_stone_diff(board) + 
        0.5 * self.bonus_moves(board)
      },
      4 => { 
        0.2 * self.store_diff(board) +
        0.4 * self.bonus_moves(board) +
        0.4 * self.highest_capture(board)
      },
      5 => {
        board[self.store as usize] as f32
      },
      6 => { 
        0.4 * self.store_diff(board) +
        0.3 * self.bonus_moves(board) +
        0.3 * self.store_hits(board)
      },
      _ => panic!("invalid heuristic type"),
    }
  }

  fn store_diff(&self, board: &[u8; 14]) -> f32 {
    board[self.store as usize] as f32 - board[self.opp_store as usize] as f32
  }

  fn bin_stone_diff(&self, board: &[u8; 14]) -> f32 {
    board[(self.range[0] as usize)..=(self.range[5] as usize)].iter().sum::<u8>() as f32 
    - board[(self.opp_range[0] as usize)..=(self.opp_range[5] as usize)].iter().sum::<u8>() as f32
  }
  
  fn bonus_moves(&self, board: &[u8; 14]) -> f32 {
    // Number of bonus moves that can be earned in the next turn
    let mut count: f32 = 0.;
    for &bin in self.range.iter() {
      if (board[bin as usize] + bin) % 14 == self.store {
        count += 1.;
      }
    }
    count
  }
  
  fn store_hits(&self, board: &[u8; 14]) -> f32 {
    // Number of moves that will hit player store
    let mut count: f32 = 0.;
    for (i, &bin) in self.range.iter().enumerate() {
      if (board[bin as usize] % 7) + (i as u8) > 6 {
        count += 1.;
      }
    }
    count
  }

  fn highest_capture(&self, board: &[u8; 14]) -> f32 {
    // Gives an approximate value for highest capture in next turn
    let mut count: f32 = 0.;
    for &bin in self.range.iter() {
      if board[bin as usize] == 0 {
        continue;
      }
      let bin_new_pos = (board[bin as usize] + bin) % 14;
      let opp_bin_pos: u8 = self.store + (7 - (bin_new_pos % 7));
      if self.range.contains(&bin_new_pos) && 
         board[bin_new_pos as usize] == 0 &&
         board[opp_bin_pos as usize] != 0 {
        count = count.max(board[opp_bin_pos as usize] as f32);
      }
    }
    count
  }
}