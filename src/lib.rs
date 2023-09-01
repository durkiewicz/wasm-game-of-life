mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

impl Universe {

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                if self.is_alive_at(neighbor_row, neighbor_col) {
                    count += 1;
                }
            }
        }
        count
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    pub fn is_alive_at(&self, row: u32, column: u32) -> bool {
        let (index, shift) = get_index_and_shift(row, column, self.width);
        self.bytes[index] & (1 << shift) != 0
    }

    pub fn tick(&mut self) {
        let mut next = self.bytes.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let is_alive = self.is_alive_at(row, col);
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (is_alive, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                set_alive_at(&mut next, row, col, self.width, next_cell);
            }
        }

        self.bytes = next;
    }

    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;
        let byte_len = (width * height + 7) / 8;

        let bytes = (0..byte_len)
            .enumerate()
            .map(|(byte_index, _)| {
                let mut byte: u8 = 0;
                for bit_index in 0..8 {
                    let index = byte_index * 8 + bit_index;
                    if  index % 2 == 0 || index % 7 == 0 {
                        byte |= 1 << (7 - bit_index);
                    }
                }
                byte
            })
            .collect();

        Universe {
            width,
            height,
            bytes,
        }
    }
}

fn get_index_and_shift(row: u32, column: u32, width: u32) -> (usize, usize) {
    let index = (row * width + column) as usize;
    (index / 8, 7 - (index % 8))
}

fn set_alive_at(bytes: &mut Vec<u8>, row: u32, column: u32, width: u32, alive: bool) {
    let (index, shift) = get_index_and_shift(row, column, width);
    if alive {
        bytes[index] |= 1 << shift;
    } else {
        bytes[index] &= !(1 << shift);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_is_alive_at(bytes: Vec<u8>, width: u32, row: u32, column: u32, expected: bool) {
        let universe = Universe {
            width,
            height: bytes.len() as u32 * 8 / width,
            bytes,
        };
        assert_eq!(universe.is_alive_at(row, column), expected);
    }

    #[test]
    fn test_is_alive_at_first_bit_when_last_bit_is_set() {
        test_is_alive_at(vec![1], 4, 0, 0, false);
    }

    #[test]
    fn test_is_alive_at_last_bit_when_last_bit_is_set() {
        test_is_alive_at(vec![1], 4, 1, 3, true);
    }

    #[test]
    fn test_is_alive_at_first_bit_when_first_bit_is_set() {
        test_is_alive_at(vec![1 << 7], 4, 0, 0, true);
    }

    #[test]
    fn test_is_alive_at_last_bit_when_first_bit_is_set() {
        test_is_alive_at(vec![1 << 7], 4, 1, 3, false);
    }

    #[test]
    fn test_tick() {
        let mut universe = Universe {
            width: 8,
            height: 5,
            bytes: vec![
                0b00000000,
                0b00000000,
                0b01110000,
                0b00000000,
                0b00000000,
            ]
        };
        universe.tick();
        assert_eq!(
            universe.bytes,
            vec![
                0b00000000,
                0b00100000,
                0b00100000,
                0b00100000,
                0b00000000,
            ]
        );
    }
}
