pub mod sudoku_number;
use std::time::SystemTime;

use crate::sudoku_number::SudokuNumber;

pub mod grid;
use crate::grid::Grid;

pub mod sudoku;
use crate::sudoku::Sudoku;

fn main() {
    let mut sudoku: Sudoku = Default::default();
    sudoku.input_sudoku();
    let time_before = SystemTime::now();
    sudoku.solve();
    let time_diff = SystemTime::now().duration_since(time_before);
    println!(
        "solved sudoku in {}s",
        time_diff
            .expect("time has gone backwards... Oops")
            .as_secs_f64()
    );
    println!("{}", sudoku);
}

#[cfg(test)]
mod tests {
    use crate::Grid;
    use crate::Sudoku;
    use crate::SudokuNumber;
    #[test]
    fn grid_solve_possible_number() {
        let mut grid = Grid::default();
        grid.numbers = [
            [
                SudokuNumber::new(7),
                SudokuNumber::new(1),
                SudokuNumber::new(2),
            ],
            [
                SudokuNumber::new(8),
                SudokuNumber::new(6),
                SudokuNumber::new(5),
            ],
            [
                SudokuNumber::new(3),
                SudokuNumber::new(4),
                SudokuNumber::new(0),
            ],
        ];
        let mut grid_correct = grid.clone();
        //  7 1 2
        //  8 6 5
        //  3 4 (9)
        grid.solve_possible_number(0, 0);
        grid.set_numbers_one_possible();
        grid_correct.numbers[2][2] = SudokuNumber::new(9);
        println!("{}", grid_correct);
        println!("{}", grid);
        assert_eq!(
            grid.numbers[2][2].get_number(),
            grid_correct.numbers[2][2].get_number()
        );
    }
    #[test]
    fn big_test() {
        let mut sudoku = Sudoku::default();
        let numbers = [
            [0, 0, 2, 0, 8, 0, 0, 6, 0],
            [0, 5, 6, 9, 1, 7, 0, 3, 0],
            [0, 4, 0, 0, 5, 0, 8, 7, 0],
            [0, 9, 0, 0, 0, 0, 6, 0, 0],
            [6, 7, 1, 0, 9, 5, 2, 0, 0],
            [0, 0, 0, 0, 2, 0, 1, 0, 0],
            [1, 6, 7, 0, 3, 0, 5, 9, 0],
            [4, 8, 0, 0, 7, 0, 3, 0, 0],
            [0, 2, 5, 4, 6, 0, 0, 0, 0],
        ];
        for x in 0..9 {
            for y in 0..9 {
                sudoku.numbers[x][y] = SudokuNumber::new(numbers[y][x])
            }
        }
        sudoku.solve();
        print!("{}",sudoku);
        assert!(sudoku.completed());
    }
}
