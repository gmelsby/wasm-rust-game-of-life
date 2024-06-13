//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_game_of_life;
extern crate wasm_bindgen_test;
use wasm_game_of_life::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut initial_universe = input_spaceship();
    initial_universe.tick();
    let expected_universe = expected_spaceship();
    assert_eq!(
        expected_universe.get_cells()
            .into_iter()
            .map(|cell| *cell > 0) 
            .collect::<Vec<bool>>(),
        initial_universe.get_cells()
            .into_iter().
            map(|cell| *cell > 0)
            .collect::<Vec<bool>>()
        );
}

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells_alive(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells_alive(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}
