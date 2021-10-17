use debug_print::debug_println;
use std::u8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SudokuNumber {
    number: u8,
    pub empty: bool,
    pub possible_numbers: [bool; 9],
}

impl std::fmt::Display for SudokuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if 9 >= self.number && self.number >= 1 {
            //debug_println!("number {} is set", self.number);
            write!(f, "    {}    |", self.number)
        } else {
            let small_rep: String = {
                let mut return_val = String::new();
                for i in 0..9 {
                    if self.possible_numbers[i] {
                        //debug_println!("return value {}", i + 1);
                        let str = (i + 1).to_string();
                        return_val.push_str(&str);
                        //debug_println!("return value {}", return_val)
                    }
                }
                return_val.clone()
            };
            //debug_println!("multiple numbers {}", small_rep);
            let mut spaces = String::new();
            for _ in 0..(9 - small_rep.len()) {
                spaces.push_str(" ");
            }
            write!(f, "{}{}|", small_rep, spaces)
        }
    }
}

impl SudokuNumber {
    pub fn new(number: u8) -> SudokuNumber {
        SudokuNumber {
            number: number,
            empty: true,
            possible_numbers: [true, true, true, true, true, true, true, true, true],
        }
    }

    pub fn get_number(self) -> u8 {
        self.number
    }
    pub fn set_number(&mut self, number: u8) {
        self.number = number;
    }

    pub fn set_numbers_one_possible(&mut self) {
        let mut counter: u8 = 0;
        for i in 0..9 {
            if self.possible_numbers[i] {
                counter += 1;
            }
        }
        debug_println!("counted {} possible numbers", counter);
        if counter == 1 && self.number == 0 {
            let mut index = 0;
            for i in 0..9 {
                if self.possible_numbers[i] {
                    index = i;
                }
            }
            self.set_number((index + 1) as u8);
        }
    }
}

impl Default for SudokuNumber {
    fn default() -> SudokuNumber {
        SudokuNumber {
            number: 0,
            empty: true,
            possible_numbers: [true, true, true, true, true, true, true, true, true],
        }
    }
}
