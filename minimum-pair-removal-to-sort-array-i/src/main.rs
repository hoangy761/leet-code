struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        let mut sum_times = 0;
        if nums.len() < 2 {
            return 0;
        }
        let mut non_descreasing = false;

        while non_descreasing == false {
            non_descreasing = true;

            if nums.len() == 2 {
                if nums[0] > nums[1] {
                    sum_times += 1;
                }
                break;
            }
            let mut min = nums[0] + nums[1];
            let mut index_to_remove = 1;

            for index in 1..(nums.len()) {
                let sum_adjacent_pair = nums[index] + nums[index - 1];
                if nums[index] < nums[index - 1] {
                    non_descreasing = false;
                }
                if sum_adjacent_pair < min {
                    min = sum_adjacent_pair;
                    index_to_remove = index;
                }
            }
            if non_descreasing {
                break;
            }
            nums[index_to_remove - 1] = min;
            nums.remove(index_to_remove);
            sum_times += 1;
        }
        sum_times
    }
}

fn main() {
    let rs = Solution::minimum_pair_removal(vec![2, 2, -1, 3, -2, 2, 1, 1, 1, 0, -1]);
    // let rs = Solution::minimum_pair_removal(vec![1, 1, 4, 4, 2, -4, -1]);
    // let rs = Solution::minimum_pair_removal(vec![1, 2, 2]);

    // let rs = Solution::minimum_pair_removal(vec![5, 2, 3, 1]);

    println!("Hello, world!:: {}", rs);
}
