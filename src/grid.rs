use debug_print::debug_println;
use std::collections::HashMap;

use crate::SudokuNumber;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Grid {
    pub numbers: [[SudokuNumber; 3]; 3],
    pub completed: bool,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            numbers: [
                [Default::default(), Default::default(), Default::default()],
                [Default::default(), Default::default(), Default::default()],
                [Default::default(), Default::default(), Default::default()],
            ],
            completed: false,
        }
    }
}

impl Grid {
    pub fn solve_one_place_grid(&mut self) {
        for place in self.get_one_places() {
            self.numbers[place.1][place.2] = SudokuNumber::new(place.0);
        }
    }

    pub fn get_one_places(&self) -> Vec<(u8, usize, usize)> {
        let mut counter: HashMap<usize, (u8, usize, usize)> = HashMap::new();
        for x in 0..3 {
            for y in 0..3 {
                for i in 0..9 {
                    if self.numbers[x][y].possible_numbers[i] {
                        let count = &mut (&mut counter).entry(i + 1).or_insert((0, x, y)).0;
                        *count += 1;
                    }
                }
            }
        }
        let mut return_value: Vec<(u8, usize, usize)> = Vec::new();
        for key in counter.clone() {
            if key.1 .0 == 1 {
                return_value.push((key.0 as u8, key.1 .1, key.1 .2))
            }
        }
        return_value
    }
    pub fn solve_possible_number(&mut self, debug_x: usize, debug_y: usize) {
        let mut set_numbers: Vec<usize> = Vec::new();
        for xx in 0..3 {
            for yy in 0..3 {
                if self.numbers[xx][yy].get_number() != 0 {
                    debug_println!(
                        "found number {} in grid x{} y{}",
                        self.numbers[xx][yy].get_number(),
                        debug_x + 1,
                        debug_y + 1
                    );
                    set_numbers.push((self.numbers[xx][yy].get_number()) as usize);
                    // adds "real" number not index
                }
            }
        }
        // works until this point
        debug_println!("grid before man ");
        debug_println!("{}", self);
        for xx in 0..3 {
            for yy in 0..3 {
                for number in &set_numbers {
                    self.numbers[xx][yy].possible_numbers[(*number - 1) as usize] = false;
                    debug_println!(
                        "set x: {} {} y: {} {}  number {} false",
                        debug_x + 1,
                        xx + 1,
                        debug_y + 1,
                        yy + 1,
                        number
                    );
                }
            }
        }
        debug_println!("grid after man");
        debug_println!("{}", self);
    }
    pub fn set_numbers_one_possible(&mut self) {
        for x in &mut self.numbers {
            for y in x {
                y.set_numbers_one_possible();
            }
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for x in self.numbers {
            for y in x {
                if let Err(e) = write!(f, "{}", y) {
                    panic!("{}", e)
                }
            }
            if let Err(e) = write!(f, "\n") {
                panic!("{}", e)
            }
        })
    }
}
