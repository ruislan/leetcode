use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        // 方法1
        // 按照规则生成数组
        // 找出数组中的最大值就行了
        if n < 2 { return n; }

        let n = n as usize;
        let mut nums = vec![0; n + 1];
        nums[0] = 0;
        nums[1] = 1;
        for i in 0..nums.len() {
            let x = i * 2;
            let y = i * 2 + 1;
            if x >= 2 && x <= n { nums[x] = nums[i]; }
            if y >= 2 && y <= n { nums[y] = nums[i] + nums[i + 1]; }
        }
        nums.into_iter().max().unwrap()
    }
}