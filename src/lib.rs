use std::str;

const DIGITS: usize = 9;
const GRID_SIZE: usize = DIGITS * DIGITS;

type Cells = [u8; GRID_SIZE];

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

impl TryFrom<Vec<u8>> for Grid {
    type Error = GridError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map(Grid::new)
            .map_err(|_| GridError::ConversionFailed)
    }
}

impl str::FromStr for Grid {
    type Err = GridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<u8> = s
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10))
            .map(|x| x.unwrap_or_default() as u8)
            .collect();

        chars.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_into_should_return_an_error_for_value_length_too_short() {
        let actual: Result<Grid, GridError> = Vec::<u8>::new().try_into();
        let expected = GridError::ConversionFailed;

        assert_eq!(actual.unwrap_err(), expected);
    }

    #[test]
    fn try_into_should_return_an_error_for_value_length_too_long() {
        let actual: Result<Grid, GridError> = vec![0; 100].try_into();
        let expected = GridError::ConversionFailed;

        assert_eq!(actual.unwrap_err(), expected);
    }
}
