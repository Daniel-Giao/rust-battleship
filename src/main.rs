use rand::Rng;
use std::{
    intrinsics::simd::simd_reduce_all,
    io::{self, Write},
};

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

    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let is_horizontal = rng.gen::<bool>();

            if self.can_place_ship(row, col, size, is_horizontal) {
                for i in 0..size {
                    let (r, c) = if is_horizontal {
                        (row, col + i)
                    } else {
                        (row + i, col)
                    };
                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r,c))
                }
                break;
            }
        }
    }

    fn can_place_ship(&self, row: usize, col: usize, size: usize, is_horizontal: bool) -> bool {
        if is_horizontal {
            if col + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty {
                    return false;
                }
            }
        } else {
            if row + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    loop {}
}

fn get_player_input() {}

fn generate_opp_move() {}
