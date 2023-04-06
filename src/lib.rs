use std::str;

const DIGITS: usize = 9;
const GRID_SIZE: usize = DIGITS * DIGITS;

#[derive(Clone, Debug, Default)]
struct Cell {
    _value: u32,
}

impl Cell {
    fn new(value: u32) -> Self {
        Self { _value: value }
    }
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        Cell::new(value)
    }
}

type Cells = [Cell; GRID_SIZE];

#[derive(Debug)]
pub struct Grid {
    _cells: Cells,
}

impl Grid {
    fn new(cells: Cells) -> Self {
        Self { _cells: cells }
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
            .map(|x| x.unwrap_or_default())
            .map(Cell::new)
            .collect();

        chars.try_into()
    }
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
}
