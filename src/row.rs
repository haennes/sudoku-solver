use crate::SudokuNumber;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Row {
    pub numbers: [SudokuNumber; 9],
    pub completed: bool,
}

impl Default for Row {
    fn default() -> Row {
        let n: SudokuNumber = Default::default();
        let n1: SudokuNumber = Default::default();
        let n2: SudokuNumber = Default::default();
        let n3: SudokuNumber = Default::default();
        let n4: SudokuNumber = Default::default();
        let n5: SudokuNumber = Default::default();
        let n6: SudokuNumber = Default::default();
        let n7: SudokuNumber = Default::default();
        let n8: SudokuNumber = Default::default();

        Row {
            numbers: [n, n1, n2, n3, n4, n5, n6, n7, n8],
            completed: false,
        }
    }
}

impl Row {
    pub fn get_one_places(&self) -> Vec<(u8, usize)> {
        let mut counter: HashMap<usize, (u8, usize)> = HashMap::new();
        for x in 0..9 {
            for i in 0..9 {
                if self.numbers[x].possible_numbers[i] {
                    let count = &mut (&mut counter).entry(i).or_insert((0, x)).0;
                }
            }
        }
        let mut return_value: Vec<(u8, usize)> = Vec::new();
        for key in counter.clone() {
            if key.1 .0 == 1 {
                return_value.push((key.0 as u8, key.1 .1));
            }
        }
        return_value
    }
    pub fn solve_one_place_row(&mut self) {
        for place in self.get_one_places() {
            self.numbers[place.1] = SudokuNumber::new(place.0);
        }
    }
}
