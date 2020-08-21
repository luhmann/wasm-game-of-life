mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// * creates smaller bundles as it does not pull in a lot of the code that makes debugging allocations easier
// @see https://github.com/rustwasm/wee_alloc
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {}

extern crate js_sys;

#[wasm_bindgen]
// * represent each enum value as an 8bit integer / ! what is the default? probably 32bit
// * this is crucial for the implementation as it ensures that each cell is represented by 1 byte
// * without this layering the unint8-array on top of memory exposed by wasm would not work
// * you could however also represent it as `u16` or `u32`, but you would need to use a `UInt16Array` or `UInt32Array`
// * when accessing the data from JS
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

impl Cell {
  fn toggle(&mut self) {
    *self = match *self {
      Cell::Dead => Cell::Alive,
      Cell::Alive => Cell::Dead,
    };
  }
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

// * `wasm_bindgen` ensures that the method is exposed to JS
// * if you add it to the implementation of a struct all public methods are exposed to JS
// * as module exports
// * you seem to need the macro on the declaration and the implementation
#[wasm_bindgen]
impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neighbor_col);
        count += self.cells[idx] as u8;
      }
    }
    count
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  /// Set the width of the universe.
  ///
  /// Resets all cells to the dead state.
  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
  }

  /// Set the height of the universe.
  ///
  /// Resets all cells to the dead state.
  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
  }

  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index(row, column);
    self.cells[idx].toggle();
  }

  pub fn tick(&mut self) {
    let _timer = utils::Timer::new("Universe::tick");
    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        // log!(
        //   "cell[{}, {}] is initially {:?} and has {} live neighbors",
        //   row,
        //   col,
        //   cell,
        //   live_neighbors
        // );

        let next_cell = match (cell, live_neighbors) {
          // Rule 1: Any live cell with fewer than two live neighbours
          // dies, as if caused by underpopulation.
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          // Rule 2: Any live cell with two or three live neighbours
          // lives on to the next generation.
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          // Rule 3: Any live cell with more than three live
          // neighbours dies, as if by overpopulation.
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          // Rule 4: Any dead cell with exactly three live neighbours
          // becomes a live cell, as if by reproduction.
          (Cell::Dead, 3) => Cell::Alive,
          // All other cells remain in the same state.
          (otherwise, _) => otherwise,
        };
        // log!("    it becomes {:?}", next_cell);
        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn new() -> Universe {
    // utils::set_panic_hook();
    let width = 128;
    let height = 128;

    let cells = (0..width * height)
      .map(|_i| {
        if js_sys::Math::random() > 0.5 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      })
      .collect();

    // * new instance of struct you can use shorthand for assigning variables to properties
    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }
}

// * second implementation-block for `Universe` without the wasm-bindings
// * this is only for testing, these functions will not be exported to javascript
// ! but will take up space when compiled?
// ! why can I not annotate these to only be included in the test environment
// #[cfg(test)]
impl Universe {
  /// Get the dead and alive values of the entire universe.
  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  /// Set cells to be alive in a universe by passing the row and column
  /// of each cell as an array.
  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index(row, col);
      self.cells[idx] = Cell::Alive;
    }
  }
}

// * implementing the `Display`-trait for a a struct provides a `to_string` method and allows it usage
// * in the `format!` and `println!`-mactos
impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}
