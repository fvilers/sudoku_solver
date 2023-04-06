use std::process;

use sudoku_solver::Grid;

const INPUT: &str = "2 5  7  6
4  96  2 
    8  45
98  74   
57 8 2 69
   63  57
75  2    
 6  51  2
3  4  5 8";

fn main() {
    let Ok(grid) = INPUT.parse::<Grid>() else {
        eprintln!("Cannot parse input string to a valid grid");
        process::exit(1);
    };

    println!("{:?}", grid);
}
