mod utils;

use std::{fmt, u8};
use rand::Rng;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                write!(f, "{}", match cell {0 => '💀', _ => '😂'})?;
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
                rng.gen_range(0, 2) as u8
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

    // basic getters
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // sends a pointer to WASM memory
    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
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
                    // Rule 4: dead cell with 3 live neighbors (no more and no fewer) comes to life
                    (0, count) if count == 3 => 1,
                    // dead cells stay dead otherwise
                    (0, count) if count != 3 => 0,
                    // Rule 1: Underpopulation
                    // Rule 3: live cell with too many neighbors (> 3) dies
                    (_age, count) if count < 2 || count > 3 => 0,
                    // Rule 2: live cell with 2-3 live neighbors continues living
                    (age, count) if age < 7 && (count == 2 || count == 3) => age + 1,
                    // If other rules do not apply, nothing happens
                    (age, _) => age,
                };
                // update next frame
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
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
                if self.cells[self.get_index(candidate_row, candidate_col)] != 0 {
                    count += 1;
                }
            }
        }
        count
    }
}