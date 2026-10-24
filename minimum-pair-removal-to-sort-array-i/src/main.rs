struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut cursor = nums.len() - 1;

        let mut sum_times = 0;
        while cursor >= 1 {
            let current = cursor;
            let prev = current - 1;

            let pair_curr = nums[current] + nums[prev];
            let pair_prev = nums[prev] + nums[prev - 1];

            if nums[current] < nums[prev] || pair_curr < pair_prev {
                sum_times += 1;
                nums[cursor - 1] = pair_curr;
                nums.remove(cursor);
                match nums.get(cursor) {
                    Some(_value) => cursor = nums.len() - 1,
                    None => cursor -= 1,
                }
            } else {
                cursor -= 1;
            }
        }

        sum_times
    }
}

fn main() {
    let rs = Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]);
    println!("Hello, world!:: {}", rs);
}
