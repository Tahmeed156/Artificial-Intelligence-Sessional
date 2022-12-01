mod ai;
mod state;
mod heuristic;

use state::State;
use heuristic::Heuristic;
use std::io::Write;

const GAMES: usize = 10;

#[allow(dead_code)]
enum GameMode {
  PlayervAI,
  AIvAI
}

fn main() {
  println!("Welcome to Mancala!\nReady player one?");

  let mut result: [i8; GAMES] = [0; GAMES];
  for i in 0..GAMES {
    let GameResult(res, p1_score, p2_score) = game(6, 6, 1, 6);
    result[i] = res;
    printc(&format!("Game {:2}: P1 - {} , P2 - {}", i + 1, p1_score, p2_score));
  }

  println!("\nP1 - {}", result.iter().filter(|&n| *n == 1).count());
  println!("P2 - {}", result.iter().filter(|&n| *n == -1).count());
  println!("Draw - {}", result.iter().filter(|&n| *n == 0).count());
}

pub struct GameResult(i8, u8, u8);

macro_rules! sleep {
  ($second: expr) => {
    std::thread::sleep(std::time::Duration::from_millis($second * 1000));
  };
}

fn game(p1_depth: u8, p2_depth: u8, p1_heuristic: u8, p2_heuristic: u8) -> GameResult {
  let mut bin: u8;
  let mut player1: bool = true;
  let result: GameResult;
  let game_mode: GameMode = GameMode::PlayervAI; // 0 - AI vs AI, 1 - Player vs AI
  // println!("P1 | depth={} , heuristic={}", p1_depth, p1_heuristic);
  // println!("P2 | depth={} , heuristic={}", p2_depth, p2_heuristic);
  let p1_heuristic: Heuristic = Heuristic::new(true, p1_heuristic);
  let p2_heuristic: Heuristic = Heuristic::new(false, p2_heuristic);

  let mut game_state: State = State::initial();
  
  loop {
    match game_mode {
      GameMode::AIvAI => {
        // print_board(game_state.board, player1);
        bin = ai::minimax(
          game_state.clone(), 
          if player1 { p1_depth } else { p2_depth },
          if player1 { &p1_heuristic } else { &p2_heuristic },
        );
        // sleep!(5);
        // println!("Player {} moved {}", if player1 { "1" } else { "2" }, bin);
      },
      GameMode::PlayervAI => {
        print_board(game_state.board, player1);
        if player1 {
          bin = player_prompt(game_state.board, player1);
        } else {
          println!("Waiting for opponent");
          sleep!(1);
          bin = ai::minimax(game_state.clone(), p2_depth, &p2_heuristic);
          println!("Opponent moved {}", bin);
        }
      }
    }

    game_state = game_state.make_move(bin);
    
    if game_state.can_capture() {
      game_state.perform_capture();
    }

    // Check end game state
    if game_state.is_terminal() {
      game_state.collect_remaining();
      if matches!(game_mode, GameMode::PlayervAI) {
        print_board(game_state.board, player1);
      }
      result = game_state.get_results();
      return result;
    }

    if game_state.is_bonus_turn() {
      // printc("> Got bonus turn!");
    } else {
      player1 = !player1;
      game_state.swap_turn();
    }
  }
}

#[allow(dead_code)]
fn print_board(board: [u8; 14], player1: bool) {
  match player1 {
    true => println!("=============== Player 1 ==============="),
    false => println!("=============== Player 2 ==============="),
  }

  // if !player1 {
  //   print!("Bins:   6 |  5 |  4 |  3 |  2 |  1      \n");
  // }
  print!("+----+----+----+----+----+----+----+----+\n");

  print!("|    ");
  for val in board[8..=13].iter().rev() {
    print!("| {:>2} ", val);
  }
  print!("|    |\n");

  print!("| {:>2} |----|----|----|----|----|----| {:>2} |\n", board[0], board[7]);

  print!("|    ");
  for val in board[1..=6].iter() {
    print!("| {:>2} ", val);
  }
  print!("|    |\n");

  print!("+----+----+----+----+----+----+----+----+\n");
  if player1 {
    print!("Bins:   1 |  2 |  3 |  4 |  5 |  6      \n");
  }
}

#[allow(dead_code)]
fn player_prompt(board: [u8; 14], player1: bool) -> u8 {
  let mut index: u8;

  loop {
    print!("Enter bin to move: ");
    std::io::stdout().flush().ok().expect("could not flush stdout");

    let mut letter: String = String::new();
    let letter_bytes: usize = std::io::stdin().read_line(&mut letter).unwrap();
    if letter_bytes > 2 {
      println!("ERR: Invalid input");
      continue;
    }

    index = letter.chars().nth(0).unwrap() as u8 - '0' as u8;
    if !player1 {
      index += 7;
    }
    if board[index as usize] == 0 {
      println!("ERR: Bin is empty");
      continue;
    }

    break;
  }

  index
}

#[allow(dead_code)]
fn printc(msg: &str) {
  println!("\x1b[0;31m{}\x1b[0m", msg);
}