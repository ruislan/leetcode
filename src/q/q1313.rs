use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // Passed 4ms 2mb
        let mut res = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            for _ in 0..nums[i] {
                res.push(nums[i + 1]);
            }
        }
        res

        // 方法2
        // Passed 8ms 2.1m
        // let mut res = Vec::new();
        // for i in (0..nums.len()).step_by(2) {
        //     res.append(&mut vec![nums[i + 1]; nums[i] as usize]);
        // }
        // res
    }
}

#[test]
fn test_q1313() {
    assert_eq!(Solution::decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
    assert_eq!(Solution::decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}