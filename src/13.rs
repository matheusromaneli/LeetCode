use std::collections::HashMap;

impl Solution {

    pub fn calc(next: char, x: char, y: char) -> i32{
        if next == x || next == y{
            -1
        } else {
            1
        }
    }

    pub fn roman_to_int(s: String) -> i32 {

        let mut result = 0;
        for i in 0..s.len() {
            let next = s.chars().nth(i+1).unwrap_or('_');
            let current = s.chars().nth(i).unwrap();
            match current {
                'I' => result += Self::calc(next, 'V', 'X'),
                'V' => result += 5,
                'X' => result += 10 * Self::calc(next, 'L', 'C'),
                'L' => result += 50,
                'C' => result += 100 * Self::calc(next, 'D', 'M'),
                'D' => result += 500,
                'M' => result += 1000,
                _ => todo!(),
            }
        }

        result
    }
}