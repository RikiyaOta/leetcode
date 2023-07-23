/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

fn main() {}

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut not_matched = 0;

        for pointer in 0..nums.len() {
            if nums[pointer] != val {
                nums[not_matched] = nums[pointer];
                not_matched += 1;
            }
        }
        
        not_matched as i32
    }
}
// @lc code=end

