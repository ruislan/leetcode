struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        // 方法1
        // 暴力解决，从1:n，直到选出每个和大于等于1的，这个数自然是最小值
        // Passed 0ms 2mb
        // let mut num = 1;
        // 'seek: loop {
        //     let mut sum = num;
        //     for i in 0..nums.len() {
        //         sum += nums[i];
        //         if sum < 1 {
        //             num += 1;
        //             continue 'seek;
        //         }
        //     }
        //     return num;
        // }

        // 方法2
        // 求出最小前缀和，然后得到最小的那个负数（如果有负数的话），那个负数的绝对值+1就是最小开始的数
        // 如果没有负数，则直接返回1即可
        let mut acc = 0;
        1 - nums.into_iter().map(|x| {
            acc += x;
            acc
        }).min().unwrap_or(0).min(0)
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::min_start_value(vec![]), 0);
    assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
    assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
    assert_eq!(Solution::min_start_value(vec![-2, -3]), 6);
}