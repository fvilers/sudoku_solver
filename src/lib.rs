use std::{fmt, str};

const DIGITS: usize = 9;
const GRID_SIZE: usize = DIGITS * DIGITS;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    value: Option<u32>,
}

impl Cell {
    fn new(value: Option<u32>) -> Self {
        Self { value }
    }
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        Cell::new(Some(value))
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.value {
            Some(v) => v.to_string(),
            None => String::from(" "),
        };

        write!(f, "{}", value)
    }
}

type Cells = [Cell; GRID_SIZE];

#[derive(Debug)]
pub struct Grid {
    cells: Cells,
}

impl Grid {
    fn new(cells: Cells) -> Self {
        Self { cells }
    }
}

#[derive(Debug, PartialEq)]
pub enum GridError {
    ConversionFailed,
}

impl TryFrom<Vec<Cell>> for Grid {
    type Error = GridError;

    fn try_from(value: Vec<Cell>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(Grid::new)
            .map_err(|_| GridError::ConversionFailed)
    }
}

impl str::FromStr for Grid {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Cell> = s
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10))
            .map(Cell::new)
            .collect();

        chars.try_into()
    }
}

// A grid line is 4 characters long for each digit + a closing character + line feed
const DISPLAY_GRID_LINE: usize = (DIGITS * 4) + 2;
// A grid is 2 lines long per row + 1 closing line
const DISPLAY_GRID: usize = DISPLAY_GRID_LINE * ((DIGITS * 2) + 1);

// TODO: improve how the grid is drawn
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(DISPLAY_GRID);

        s.push_str("┏━━━┯━━━┯━━━┳━━━┯━━━┯━━━┳━━━┯━━━┯━━━┓\n");

        for y in 0..DIGITS {
            s.push_str("┃ ");

            for x in 0..DIGITS {
                let index = coords_to_index(x, y);
                let cell_value = self.cells[index];
                let delimiter = if (x + 1) % 3 != 0 { '│' } else { '┃' };

                s.push_str(&format!("{} {} ", cell_value, delimiter));
            }

            s.push('\n');

            if y < DIGITS - 1 {
                let line_separator = if (y + 1) % 3 != 0 {
                    "┠───┼───┼───╂───┼───┼───╂───┼───┼───┨"
                } else {
                    "┣━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┫"
                };
                s.push_str(&format!("{}\n", line_separator));
            }
        }

        s.push_str("┗━━━┷━━━┷━━━┻━━━┷━━━┷━━━┻━━━┷━━━┷━━━┛\n");

        write!(f, "{}", s)
    }
}

fn coords_to_index(x: usize, y: usize) -> usize {
    (y * DIGITS) + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_into_should_return_an_error_for_value_length_too_short() {
        let actual: Result<Grid, GridError> = Vec::<Cell>::new().try_into();
        let expected = GridError::ConversionFailed;

        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn try_into_should_return_an_error_for_value_length_too_long() {
        let actual: Result<Grid, GridError> = vec![Cell::default(); 100].try_into();
        let expected = GridError::ConversionFailed;

        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn coords_to_index_should_return_0_for_0_0() {
        let actual = coords_to_index(0, 0);
        let expected = 0;

        assert_eq!(actual, expected);
    }

    #[test]
    fn coords_to_index_should_return_42_for_6_4() {
        let actual = coords_to_index(6, 4);
        let expected = 42;

        assert_eq!(actual, expected);
    }

    #[test]
    fn coords_to_index_should_return_80_for_8_8() {
        let actual = coords_to_index(8, 8);
        let expected = 80;

        assert_eq!(actual, expected);
    }
}
