use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Dead
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    // a digital frontier
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    /// Create a new, empty grid
    pub fn empty(x_size: usize, y_size: usize) -> Grid {
        let row: Vec<Cell> = (0..y_size).map(|_| Cell::Dead).collect();
        Grid {
            grid: (0..x_size).map(|_| row.clone()).collect(),
        }
    }

    /// Create a new grid, with the given probability of a node being alive
    pub fn random(x_size: usize, y_size: usize, probability: f64) -> Grid {
        let mut rng = SmallRng::from_entropy();
        Grid {
            grid: (0..x_size)
                .map(|_| {
                    (0..y_size)
                        .map(|_| {
                            if rng.gen_bool(probability) {
                                Cell::Alive
                            } else {
                                Cell::Dead
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    /// One game of life iteration
    pub fn tick(&self) -> Grid {
        Grid {
            grid: self
                .grid
                .iter()
                .enumerate()
                .map(|(x, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(y, cell)| {
                            let neighbours = self.neighbours(x, y);
                            match cell {
                                Cell::Alive if neighbours >= 4 => Cell::Dead,
                                Cell::Alive if neighbours <= 1 => Cell::Dead,
                                Cell::Dead if neighbours == 3 => Cell::Alive,
                                cell => *cell,
                            }
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn is_alive(&self, x: isize, y: isize) -> bool {
        match (usize::try_from(x), usize::try_from(y)) {
            (Ok(x), Ok(y)) => self
                .grid
                .get(x)
                .and_then(|r| r.get(y))
                .map(|c| *c == Cell::Alive)
                .unwrap_or_default(),
            _ => false,
        }
    }

    fn neighbours(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;

        for i in (-1)..=1 {
            for j in (-1)..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if self.is_alive((x as isize) + i, (y as isize) + j) {
                    sum += 1
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_grid(chars: &str) -> Grid {
        Grid {
            grid: String::from(chars)
                .lines()
                .map(|r| {
                    r.split_whitespace()
                        .map(|c| if c == "*" { Cell::Alive } else { Cell::Dead })
                        .collect()
                })
                .collect(),
        }
    }

    #[test]
    fn grid() {
        let expected = Grid {
            grid: vec![
                vec![Cell::Alive, Cell::Dead, Cell::Alive],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Alive, Cell::Alive, Cell::Dead],
            ],
        };

        assert_eq!(
            expected,
            make_grid(
                "* . *
                 . . .
                 * * . "
            )
        );
    }

    #[test]
    fn test_count_neighbours() {
        assert_eq!(
            8,
            make_grid(
                "* * *
                 * . *
                 * * *"
            )
            .neighbours(1, 1)
        );

        assert_eq!(
            0,
            make_grid(
                ". . *
                 . . *
                 * * *"
            )
            .neighbours(0, 0)
        );

        assert_eq!(
            2,
            make_grid(
                ". . *
                 . . *
                 * * *"
            )
            .neighbours(2, 2)
        );

        assert_eq!(
            3,
            make_grid(
                "* * *
                 . . *
                 * * *"
            )
            .neighbours(0, 1)
        )
    }

    #[test]
    fn too_many_neighbours() {
        assert_eq!(
            make_grid(
                "* . *
                 . . .
                 * . *"
            ),
            make_grid(
                "* * *
                 * * *
                 * * *"
            )
            .tick()
        );
    }

    #[test]
    fn lonely() {
        assert_eq!(
            make_grid(
                ". . .
                 . . .
                 . . ."
            ),
            make_grid(
                "* * .
                 . . .
                 . . ."
            )
            .tick()
        );

        assert_eq!(
            make_grid(
                ". . .
                 . . .
                 . . ."
            ),
            make_grid(
                ". . .
                 . * .
                 . . ."
            )
            .tick()
        );
    }

    #[test]
    fn new_life() {
        assert_eq!(
            make_grid(
                "* * .
                 * * .
                 . . ."
            ),
            make_grid(
                "* * .
                 * . .
                 . . ."
            )
            .tick()
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            make_grid(
                ". . . .
                 . . . .
                 . . . ."
            ),
            Grid::empty(3, 4)
        )
    }

    #[test]
    fn random() {
        let grid = Grid::random(4, 5, 0.5);
        assert_eq!(4, grid.grid.len());
        for row in grid.grid {
            assert_eq!(5, row.len())
        }
    }
}
