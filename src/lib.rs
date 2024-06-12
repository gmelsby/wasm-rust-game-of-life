mod utils;

use std::fmt;
use rand::Rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", match cell {Cell::Dead => '💀', Cell::Alive => '😂'})?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

# [wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let mut rng = rand::thread_rng();
        let cells = (0..width * height)
            .map(|_| {
                if rng.gen_range(0, 2) < 1 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
            
            Universe {
                width,
                height,
                cells,
            }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    // passes time in the universe
    pub fn tick(&mut self) {
        // make copy to modify
        let mut next = self.cells.clone();

        for row in 0..self.height{
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbor_count = self.live_neighbor_count(row, col);
                // match expression determines what happens to the cell
                let next_cell = match (cell, neighbor_count) {
                    // Rule 1: Underpopulation
                    (Cell::Alive, count) if count < 2 => Cell::Dead,
                    // Rule 2: live cell with 2-3 live neighbors continues living
                    (Cell::Alive, count) if count == 2 || count == 3 => Cell::Alive,
                    // Rule 3: live cell with too many neighbors (> 3) dies
                    (Cell::Alive, count) if count > 3 => Cell::Dead,
                    // Rule 4: dead cell with 3 live neighbors (no more and no fewer) comes to life
                    (Cell::Dead, count) if count == 3 => Cell::Alive,
                    // If other rules do not apply, nothing happens
                    (boring_cell, _) => boring_cell,
                };
                // update next frame
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for x in [self.width - 1, 0, 1].iter().cloned() {
            for y in [self.height -1, 0, 1].iter().cloned() {
                // case where we are looking at the cell we're finding neighbors for
                if x == 0 && y == 0 {
                    continue;
                }

                let candidate_row = (row + y) % self.height;
                let candidate_col = (col + x) % self.width;
                count += self.cells[self.get_index(candidate_row, candidate_col)] as u8
            }
        }
        count
    }
}