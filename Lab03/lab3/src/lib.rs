use random::Source;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn date_now() -> f64;
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cell {
    Dead = 0,
    Dying = 1,
    Alive = 2,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dying,
            Cell::Dying => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let (width, height) = (64, 64);
        let mut source = random::default(date_now() as u64);

        let cells = (0..width * height)
            .map(|_| {
                if source.read::<f64>() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Self {
            width,
            height,
            cells,
        }
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

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn count_live_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for i in [self.height - 1, 0, 1] {
            for j in [self.width - 1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                let idx = self.get_index((row + i) % self.height, (col + j) % self.width);
                if self.cells[idx] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                cells[idx] = match self.cells[idx] {
                    Cell::Alive => Cell::Dying,
                    Cell::Dying => Cell::Dead,
                    Cell::Dead => {
                        if self.count_live_neighbours(row, col) == 2 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                }
            }
        }
        self.cells = cells;
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}
