use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        // max : 0
        // middle : 1
        // min : 2
        let mut top_maximum: HashMap<&str, i32> = HashMap::new();

        if nums.len() <= 2 {
            return *nums.iter().max().unwrap();
        }
        for curr in nums {
            let max = *top_maximum.entry("max").or_insert_with(|| curr);

            if max == curr {
                continue;
            }
            if curr > max {
                // update max = curr
                top_maximum.insert("max", curr);
                // middle = max
                match top_maximum.get("middle") {
                    Some(value) => {
                        top_maximum.insert("min", *value);
                    }
                    None => (),
                };
                top_maximum.insert("middle", max);

                continue;
            }

            let mildle = *top_maximum.entry("middle").or_insert_with(|| curr);

            if curr == mildle {
                continue;
            }

            if curr > mildle {
                // update middle = curr
                // update min = old_middle
                match top_maximum.get("middle") {
                    Some(value) => {
                        top_maximum.insert("min", *value);
                    }
                    None => (),
                };
                top_maximum.insert("middle", curr);

                continue;
            }

            let min = *top_maximum.entry("min").or_insert_with(|| curr);

            if curr <= min {
                continue;
            }

            if curr > min {
                top_maximum.insert("min", curr);
            }
        }

        let rs = match top_maximum.get("min") {
            Some(value) => value,
            None => top_maximum.get("max").unwrap(),
        };

        *rs
    }
}

fn main() {
    let rs = Solution::third_max(vec![1, 2]);
    println!("Hello, world!::{rs}");
}
