use std::fmt;
use std::fmt::Display;
/*
INFO: GAME LOGIC
Any live cell with fewer than two live neighbours dies
Any live cell with two or three live neighbours lives on
Any live cell with more than three live neighbours dies
Any dead cell with exactly three live neighbours becomes a live cell
*/

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[derive(Debug,PartialEq, Eq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// logic
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        Universe {
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    /// set the selected vector of cells to alive
    ///
    /// * `cells`: tuples of cells (row, col)
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (x, y) in cells.iter().clone() {
            let index = self.get_index(*x, *y);
            self.cells[index] = Cell::Alive;
        }
    }

    /// Get the index of the selected cell in the universe
    ///
    /// * `x`: x coord, starting 0
    /// * `y`: y coord, starting 0
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn live_neighbour_count(&self, x: u32, y: u32) -> u8 {
        // upper row: index - row -1 to index - row + 1
        // lower row: index + row +1 to index + row - 1
        let mut sum = 0u8;
        // self - 1 avoids out of bounds when x is 0 (0 - 1)
        for delta_x in [self.width - 1, 0, 1].iter().cloned() {
            for delta_y in [self.height - 1, 0, 1].iter().cloned() {
                // self
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                let neighbor_x = (x + delta_x) % self.width;
                let neighbor_y = (y + delta_y) % self.height;
                let index = self.get_index(neighbor_x, neighbor_y);
                sum += self.cells[index] as u8;
            }
        }
        sum
    }

    pub fn process(&mut self) {
        let mut next = self.cells.clone();
        for x in 0..self.height {
            for y in 0..self.width {
                let index = self.get_index(x, y);
                let state = self.cells[index];
                let neighbour_count = self.live_neighbour_count(x, y);
                next[index] = match (state, neighbour_count) {
                    (Cell::Alive, count) if count < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, count) if count > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (other, _) => other
                }
            }
        }
        self.cells = next;
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        // makeshift of lines()
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼',
                };
                // writing into row
                write!(f, "{}", symbol)?;
            }
            // newline after writing row
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbour() {
        let mut uni = Universe::new(10,10);
        uni.set_cells(&[(4,4),(4,5),(4,6),(5,5),(5,6)]);
        assert_eq!(uni.live_neighbour_count(5, 4), 3);
        assert_eq!(uni.live_neighbour_count(5, 5), 4);
    }

    #[test]
    fn tick() {
        let mut uni = Universe::new(10,10);
        uni.set_cells(&[(4,4),(5,4),(6,4),(5,5),(6,5)]);

        let mut uni2 = Universe::new(10,10);
        uni2.set_cells(&[(4,4),(6,4),(5,3),(4,5),(6,5)]);

        uni.process();
        assert_eq!(uni,uni2);
    }
}
