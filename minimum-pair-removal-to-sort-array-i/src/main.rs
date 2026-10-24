struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        let mut sum_times = 0;
        let mut need_loop = true;
        if nums.len() < 2 {
            return 0;
        }

        while need_loop {
            need_loop = false;
            if nums.len() == 2 {
                if nums[0] > nums[1] {
                    sum_times += 1;
                }
                break;
            }
            if nums.len() == 4 {
                println!("hi");
            }
            let mut min = nums[0] + nums[1];
            let mut index_to_remove = 0;

            let mut min_compare = nums[0] + nums[1];
            let mut index_to_remove_compare = 0;
            for index in 1..(nums.len()) {
                let sum_adjacent_pair = nums[index] + nums[index - 1];
                if nums[index] < nums[index - 1] {
                    min_compare = sum_adjacent_pair;
                    need_loop = true;
                    index_to_remove_compare = index;
                }
                if sum_adjacent_pair <= min {
                    if index == 1 {
                        continue;
                    }
                    min = sum_adjacent_pair;
                    need_loop = true;
                    index_to_remove = index;
                }
            }
            if need_loop {
                if index_to_remove > 0 {
                    nums[index_to_remove - 1] = min;
                    nums.remove(index_to_remove);
                    sum_times += 1;
                } else if index_to_remove_compare > 0 {
                    nums[index_to_remove_compare - 1] = min_compare;
                    nums.remove(index_to_remove_compare);
                    sum_times += 1;
                }
            }
        }
        sum_times
    }
}

fn main() {
    let rs = Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]);
    println!("Hello, world!:: {}", rs);
}
