struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut len_nums = nums.len();
        if len_nums == 0 {
            return 0;
        };
        let mut right_index = len_nums - 1;

        let mut left_index = 0;
        if left_index == right_index {
            if nums[left_index] == val {
                return 0;
            } else {
                return 1;
            }
        }
        while left_index <= right_index {
            if nums[left_index] == val {
                while right_index >= left_index && nums[right_index] == val {
                    if right_index == 0 {
                        return (len_nums - 1) as i32;
                    }
                    right_index -= 1;
                    len_nums -= 1;
                }
                if right_index < left_index {
                    return len_nums as i32;
                }

                (nums[left_index], nums[right_index]) = (nums[right_index], nums[left_index]);

                len_nums -= 1;
                if right_index == 0 {
                    return len_nums as i32;
                }
                right_index -= 1;
            }
            left_index += 1;
        }
        len_nums as i32
    }
}

fn main() {
    let mut arr = vec![4, 5];
    let rs = Solution::remove_element(&mut arr, 5);
    println!("Hello, world! {}", rs);
}

// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

// Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.
// Custom Judge:

// The judge will test your solution with the following code:

// int[] nums = [...]; // Input array
// int val = ...; // Value to remove
// int[] expectedNums = [...]; // The expected answer with correct length.
//                             // It is sorted with no values equaling val.

// int k = removeElement(nums, val); // Calls your implementation

// assert k == expectedNums.length;
// sort(nums, 0, k); // Sort the first k elements of nums
// for (int i = 0; i < actualLength; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.

// Example 1:

// Input: nums = [3,2,2,3], val = 3
// Output: 2, nums = [2,2,_,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 2.
// It does not matter what you leave beyond the returned k (hence they are underscores).
// Example 2:

// Input: nums = [0,1,2,2,3,0,4,2], val = 2
// Output: 5, nums = [0,1,4,0,3,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums containing 0, 0, 1, 3, and 4.
// Note that the five elements can be returned in any order.
// It does not matter what you leave beyond the returned k (hence they are underscores).

// Constraints:

// 0 <= nums.length <= 100
// 0 <= nums[i] <= 50
// 0 <= val <= 100
