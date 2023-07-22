/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

fn main() {}

struct Solution {}

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if m == 0 {
            nums1.copy_from_slice(&nums2);
            return;
        }

        if n == 0 {
            return;
        }

        // でもこれわざわざ別で用意する必要ないかも？
        let mut result: Vec<i32> = vec![0; (m + n) as usize];
        let mut i1: usize = 0;
        let mut i2: usize = 0;

        while i1 + i2 < (m + n) as usize {
            println!("i1: {}, i2: {}", i1, i2);

            if i1 == m as usize {
                println!("i1 == m as usize");
                result[i1 + i2] = nums2[i2];
                i2 += 1;
                continue;
            }

            if i2 == n as usize {
                println!("i2 == n as usize");
                result[i1 + i2] = nums1[i1];
                i1 += 1;
                continue;
            }

            if nums1[i1 as usize] < nums2[i2 as usize] {
                println!("nums1[i1 as usize] < nums2[i2 as usize]");
                result[i1 + i2] = nums1[i1];
                i1 += 1;
            } else {
                println!("nums1[i1 as usize] >= nums2[i2 as usize]");
                result[i1 + i2] = nums2[i2];
                i2 += 1;
            }
        }

        nums1.copy_from_slice(&result);
    }
}
// @lc code=end
