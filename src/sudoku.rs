use crate::Grid;
use crate::SudokuNumber;
use debug_print::debug_println;
use std::io::stdin;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Sudoku {
    pub numbers: [[SudokuNumber; 9]; 9],
    pub completed: bool,
}

impl Default for Sudoku {
    fn default() -> Sudoku {
        Sudoku {
            numbers: [
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            completed: false,
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut grid = "".to_string();
        for y in 0..9 {
            let mut row = "".to_string();
            for x in 0..9 {
                let string = format!("{}", self.numbers[x][y]);
                //debug_println!("fmt_Sudoku{}", &string);
                row.push_str(&string);
            }
            row += "\n";
            grid.push_str(&row);
        }
        write!(f, "{}", grid)
    }
}

impl Sudoku {
    pub fn input_sudoku(&mut self) {
        println!("Input all numbers top left to top right to bottom");
        println!("press ENTER to proceed");
        let mut skip = String::new();
        let err = stdin().read_line(&mut skip);
        match err {
            Ok(_) => {}
            Err(e) => panic!("unable to read input {}", e),
        }
        for y in 0..9 {
            for x in 0..9 {
                let mut number_input = String::new();
                stdin()
                    .read_line(&mut number_input)
                    .expect("Oops something went wrong with your Input");
                let number_input = number_input.trim();
                //debug_println!("input {}input",number_input);
                if number_input != "".to_string() {
                    match number_input.parse::<u8>() {
                        Ok(number_converted) => {
                            self.numbers[x][y] = SudokuNumber::new(number_converted)
                        }
                        Err(e) => println!("Error {}", e),
                    }
                } else {
                    self.numbers[x][y] = SudokuNumber::new(0)
                }
            }
        }
    }
    pub fn update_completed(&mut self) {
        self.completed = self.completed();
    }
    pub fn completed(self) -> bool {
        for x in self.numbers {
            for y in x {
                if y.get_number() == 0 {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn solve(&mut self) {
        println!("solving started");
        let mut previous_iteration = self.numbers;
        loop {
            let mut previous_iteration_one = self.numbers;
            loop {
                for i in 0..9 {
                    for x in 0..9 {
                        for y in 0..9 {
                            self.solve_possible_number(x, y, i);
                        }
                    }
                    //debug_println!("solving for {}", i + 1)
                }

                self.set_numbers_one_possible();
                if previous_iteration_one == self.numbers {
                    debug_println!("nothing changed!!!!");
                    break;
                }

                previous_iteration_one = self.numbers;
            }

            let mut previous_iteration_two = self.numbers;
            loop {
                debug_println!("solve possible number grids"); // not working
                self.solve_possible_number_grids();
                self.set_numbers_one_possible();
                if previous_iteration_two == self.numbers {
                    debug_println!("nothing changed!!!!");
                    break;
                }

                previous_iteration_two = self.numbers;
            }

            let mut previous_iteration_three = self.numbers;
            loop {
                debug_println!("solve one place grids");
                self.solve_one_place_grids();
                self.set_numbers_one_possible();
                if previous_iteration_three == self.numbers {
                    debug_println!("noting changed!");
                    break;
                }
                previous_iteration_three = self.numbers
            }
            if previous_iteration == self.numbers {
                debug_println!("nothing changed!");
                break;
            }
            previous_iteration = self.numbers;
        }
    }
    fn solve_possible_number(&mut self, x: usize, y: usize, number: usize) {
        // for easy difficulty
        // looking if number is possible in the given cell
        //x
        for xx in 0..9 {
            if x == xx {
                continue;
            }
            if self.numbers[xx][y].get_number() == (number as u8) + 1 {
                //debug_println!("Hit in easy difficulty x");
                self.numbers[x][y].possible_numbers[number] = false;
            }
        }
        //y
        for yy in 0..9 {
            if y == yy {
                continue;
            }
            if self.numbers[x][yy].get_number() == (number as u8) + 1 {
                //debug_println!("Hit in easy difficulty y");
                self.numbers[x][y].possible_numbers[number] = false;
            }
        }
    }

    fn solve_possible_number_grids(&mut self) {
        let mut grids = self.get_grids();
        let mut x_counter = 0;
        for x in &mut grids {
            let mut y_counter = 0;
            for y in x {
                y.solve_possible_number(x_counter, y_counter);
                y_counter += 1;
            }
            x_counter += 1;
        }
        debug_println!("poss num grids");
        debug_println!("{}", self);
        self.from_grids(grids);
    }

    fn solve_one_place_grids(&mut self) {
        let mut grids = self.get_grids();
        for x in &mut grids {
            for y in x {
                y.solve_one_place_grid();
            }
        }
        self.from_grids(grids);
    }

    fn get_grids(&self) -> [[Grid; 3]; 3] {
        let mut grids: [[Grid; 3]; 3] = Default::default();
        for x in 0..3 {
            for y in 0..3 {
                let mut grid: Grid = Default::default();
                for xx in 0..3 {
                    for yy in 0..3 {
                        grid.numbers[xx][yy] = self.numbers[x * 3 + xx][y * 3 + yy];
                    }
                }
                grids[x][y] = grid;
            }
        }
        return grids;
    }

    fn set_numbers_one_possible(&mut self) {
        //sets number if only one is true in possible_numbers
        for x in &mut self.numbers {
            for y in x {
                y.set_numbers_one_possible();
            }
        }
    }
    fn from_grids(&mut self, val: [[Grid; 3]; 3]) {
        let mut x_counter = 0;
        for x in val {
            let mut y_counter = 0;
            for y in x {
                let mut xx_counter = 0;
                for xx in y.numbers {
                    let mut yy_counter = 0;
                    for yy in xx {
                        self.numbers[x_counter * 3 + xx_counter][y_counter * 3 + yy_counter] = yy;
                        yy_counter += 1;
                    }
                    xx_counter += 1;
                }
                y_counter += 1;
            }
            x_counter += 1;
        }
    }
}
