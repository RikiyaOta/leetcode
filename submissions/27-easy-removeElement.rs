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
        let mut k = nums.len();
        
        for i in 0..nums.len() {
            if nums[i] == val {
                nums[i] = -1;
                k -= 1;
            }
        }

        nums.sort();
        nums.reverse();
        
        k as i32
    }
}
// @lc code=end

