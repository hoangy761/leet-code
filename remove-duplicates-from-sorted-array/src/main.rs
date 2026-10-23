struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len < 1 || len > 3 * 10_usize.pow(4) {
            return -1;
        }

        let mut total_unique_num = 1;
        let mut check_point = 1;

        for index in 1..len {
            if check_point == len {
                return total_unique_num;
            }
            if nums[index] <= nums[index - 1] {
                while check_point < len {
                    check_point += 1;
                    if check_point == len {
                        return total_unique_num;
                    }

                    if nums[check_point] > nums[index - 1] {
                        //swap
                        (nums[index], nums[check_point]) = (nums[check_point], nums[index]);
                        total_unique_num += 1;
                        break;
                    }
                }
            } else {
                total_unique_num += 1;
            }
        }

        //swap
        // total_unique_num +=1

        total_unique_num
    }
}

fn main() {
    println!(
        "[1,1,2] {}",
        Solution::remove_duplicates(&mut vec![1, 1, 2])
    );
    println!("[1,2] {}", Solution::remove_duplicates(&mut vec![1, 1, 2]));
    println!(
        "[0,0,1,1,1,2,2,3,3,4] {}",
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
    );
}
