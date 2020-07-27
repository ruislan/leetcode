use crate::q::Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        // 方法1
        // 先按照公式nums[i] = start + 2*i求出数组nums
        // 然后再依次做XOR操作
        // let nums: Vec<i32> = (0..n).map(|i| start + 2 * i).collect();
        // (1..n as usize).fold(nums[0], |acc, i| nums[i] ^ acc);
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::xor_operation(0, 0), 0);
    assert_eq!(Solution::xor_operation(5, 0), 8);
    assert_eq!(Solution::xor_operation(4, 3), 8);
    assert_eq!(Solution::xor_operation(1, 7), 7);
    assert_eq!(Solution::xor_operation(10, 5), 2);
}