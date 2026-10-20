struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let mut open_scores = Vec::with_capacity(s.len() / 2);
        for c in s.chars() {
            let score = Self::bracket_score(c);
            if score > 0 {
                open_scores.push(score);
            } else if open_scores.pop() != Some(-score) {
                return false;
            }
        }
        open_scores.is_empty()
    }

    fn bracket_score(c: char) -> i8 {
        match c {
            '(' => 1,
            ')' => -1,
            '[' => 2,
            ']' => -2,
            '{' => 3,
            '}' => -3,
            _ => 0,
        }
    }
}

fn main() {
    println!("Hello, world!::{}", Solution::is_valid("{}]p".to_string()));
}
