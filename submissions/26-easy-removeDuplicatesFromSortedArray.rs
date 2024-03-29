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
        // ↓ソートされてるという条件を使ってないので、HashSet を無駄に使うことになってしまった。
        //let mut unique_nums_set: HashSet<i32> = HashSet::new();
        //let mut pointer = 0;
        //for i in 0..nums.len() {
        //    if !unique_nums_set.contains(&nums[i]) {
        //        unique_nums_set.insert(nums[i]);
        //        nums[pointer] = nums[i];
        //        pointer += 1;
        //    }
        //}

        //unique_nums_set.len() as i32

        // Sorted Array という性質を使えばもっと上手くかける
        if nums.len() == 0 {
            return 0;
        }

        let mut pointer = 0;
        for i in 1..nums.len() {
            if nums[pointer] != nums[i] {
                pointer += 1;
                nums[pointer] = nums[i];
            }
        }

        (pointer + 1) as i32
    }
}
// @lc code=end
