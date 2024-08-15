use rand::Rng;
use std::io::{self, Write};

const BOARD_SIZE: usize = 10;

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, PartialEq)]

enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

impl Board {
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }
}

fn main() {
    loop {}
}

fn get_player_input() {}

fn generate_opp_move() {}
