use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        // 方法1
        // AC 0ms 2mb
        // let mut max = 0;
        // let mut c = false;
        // let mut count = 0;
        // for n in nums {
        //     if n == 1 {
        //         if c {
        //             count += 1;
        //         } else {
        //             c = true;
        //             count = 1;
        //         }
        //         if count > max {
        //             max = count;
        //         }
        //     } else {
        //         c = false;
        //         count = 0;
        //     }
        // }
        // max

        // 方法2
        // 方法1是1年前做的了，现在来看挺复杂
        // 这次我们用滑动窗口来解决一下
        // AC 0~4ms 2mb
        let mut answer = 0;
        let mut lo = 0;
        for hi in 0..nums.len() {
            if nums[hi] != 1 {
                lo = hi + 1;
            }
            answer = answer.max(hi - lo + 1);
        }
        answer as i32
    }
}