mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    fn live_neighbor_count(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for delta_y in [self.height - 1, 0, 1].iter().cloned() {
            for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                let neighbor_x = (x + delta_x) % self.width;
                let neighbor_y = (y + delta_y) % self.height;
                let idx = self.get_index(neighbor_x, neighbor_y);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let (width, height): (u32, u32) = (64, 64);
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
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

    pub fn tick(&mut self) {
        let mut next_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];
                let neighbor_count = self.live_neighbor_count(x, y);

                let next_cell = match (cell, neighbor_count) {
                    (Cell::Alive, cnt) if cnt < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, cnt) if cnt > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next_cells[idx] = next_cell;
            }
        }
        self.cells = next_cells;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
