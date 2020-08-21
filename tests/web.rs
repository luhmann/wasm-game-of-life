//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

// this configures the tests to run in a browser
// this will not run the tests in node
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
  let mut universe = Universe::new();
  universe.set_width(6);
  universe.set_height(6);
  universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
  universe
}

/*
"The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build. This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included."
@see https://doc.rust-lang.org/book/ch11-03-test-organization.html

the functions will not be included in the compilation of the final outputwe
*/
#[cfg(test)]
pub fn expected_spaceship() -> Universe {
  let mut universe = Universe::new();
  universe.set_width(6);
  universe.set_height(6);
  universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
  universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
  // Let's create a smaller Universe with a small spaceship to test!
  let mut input_universe = input_spaceship();

  // This is what our spaceship should look like
  // after one tick in our universe.
  let expected_universe = expected_spaceship();

  // Call `tick` and then see if the cells in the `Universe`s are the same.
  input_universe.tick();
  assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
