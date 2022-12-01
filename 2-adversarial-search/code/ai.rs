use crate::state::State;
use crate::heuristic::Heuristic;

pub fn minimax(state: State, depth: u8, heuristic: &Heuristic) -> u8 {
  let mut max_bin: u8 = 0; // random initialization to remove error
  let mut max_utility: f32 = f32::NEG_INFINITY;
  let mut new_utility: f32;
  for bin in state.get_moves() {
    let mut new_state: State = state.make_move(bin);
    if new_state.can_capture() {
      new_state.perform_capture();
    }
    // println!("Root exploring bin {} at depth -{}", bin, depth);
    if new_state.is_terminal() {
      new_state.collect_remaining();
      new_utility = heuristic.get_value(&new_state.board);
    } else if new_state.is_bonus_turn() {
      new_utility = max_value(new_state, heuristic, depth - 1, f32::NEG_INFINITY, f32::INFINITY);
    } else {
      new_state.swap_turn();
      new_utility = min_value(new_state, heuristic, depth - 1, f32::NEG_INFINITY, f32::INFINITY);
    }
    // println!("Root got {} max_utility is {}", new_utility, max_utility);
    if new_utility > max_utility {
      max_bin = bin;
      max_utility = new_utility;
    }
  }
  // println!("{}", max_utility);
  max_bin
}

fn min_value(state: State, heuristic: &Heuristic, depth: u8, alpha: f32, mut beta: f32) -> f32 {
  if depth == 0 {
    return heuristic.get_value(&state.board);
  }

  let mut utility: f32 = f32::INFINITY;

  for bin in state.get_moves() {
    let mut new_state: State = state.make_move(bin);
    if new_state.can_capture() {
      new_state.perform_capture();
    }
    // println!("{} exploring bin {} at depth -{}", "Min", bin, depth);
    let new_utility: f32;
    if new_state.is_terminal() {
      new_state.collect_remaining();
      new_utility = heuristic.get_value(&new_state.board);
    } else if new_state.is_bonus_turn() {
      new_utility = min_value(new_state, heuristic, depth - 1, alpha, beta);
    } else {
      new_state.swap_turn();
      new_utility = max_value(new_state, heuristic, depth - 1, alpha, beta);
    }
    utility = utility.min(new_utility);
    // println!("{} at depth -{}, bin {} gets utility {}", "Min", depth, bin, new_utility);
    if utility <= alpha {
      return utility;
    }
    beta = beta.min(utility);
  }

  utility
}

fn max_value(state: State, heuristic: &Heuristic, depth: u8, mut alpha: f32, beta: f32) -> f32 {
  if depth == 0 {
    return heuristic.get_value(&state.board);
  }

  let mut utility: f32 = f32::NEG_INFINITY;
  
  for bin in state.get_moves() {
    let mut new_state: State = state.make_move(bin);
    if new_state.can_capture() {
      new_state.perform_capture();
    }
    // println!("{} exploring bin {} at depth -{}", "Max", bin, depth);
    let new_utility: f32;
    if new_state.is_terminal() {
      new_state.collect_remaining();
      new_utility = heuristic.get_value(&new_state.board);
    } else if new_state.is_bonus_turn() {
      new_utility = max_value(new_state, heuristic, depth - 1, alpha, beta);
    } else {
      new_state.swap_turn();
      new_utility = min_value(new_state, heuristic, depth - 1, alpha, beta);
    }
    utility = utility.max(new_utility);
    // println!("{} at depth -{}, bin {} gets utility {}", "Max", depth, bin, new_utility);
    if utility >= beta {
      return utility;
    }
    alpha = alpha.max(utility);
  }

  utility
}