/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

fn main() {}

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut pointer = 0;
        for i in 1..nums.len() {
            if nums[pointer] == nums[i] {
                if pointer == 0 {
                    pointer += 1;
                }

                if pointer > 0 && nums[pointer - 1] != nums[pointer] {
                    pointer += 1;
                    nums[pointer] = nums[i];
                }
            } else {
                pointer += 1;
                nums[pointer] = nums[i];
            }
        }

        (pointer + 1) as i32
    }
}
// @lc code=end
