/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 */

fn main() {}

struct Solution {}

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_nums_set: HashSet<i32> = HashSet::new();
        let mut pointer = 0;
        for i in 0..nums.len() {
            if !unique_nums_set.contains(&nums[i]) {
                unique_nums_set.insert(nums[i]);
                nums[pointer] = nums[i];
                pointer += 1;
            }
        }

        unique_nums_set.len() as i32
    }
}
// @lc code=end
