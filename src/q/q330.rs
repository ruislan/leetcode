use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        // 方法1
        // dp
        // dp = [0,1,...,n]
        // 然后迭代nums，如果i - nums[i] 在dp[i]存在，就说明设置dp[i]为真
        // 这说明我们能够通过加法算出这个数字
        // 当nums迭代完成，找出dp中第一个不存在的数字继续进行上述迭代
        // 直到dp中所有的数字都存在
        // Not Passed 数字不是很大的可以解决，大数字面前就会超时
        
        // let n = n as usize;
        // let mut que: std::collections::VecDeque<i32> = nums.into_iter().map(|x| x).collect();
        //
        // let mut dp = vec![false; n + 1];
        // dp[0] = true;
        //
        // let mut answer = 0;
        // if que.is_empty() {
        //     que.push_back(1);
        //     answer += 1;
        // }
        //
        // let mut max = 0;
        // while !que.is_empty() {
        //     for _ in 0..que.len() {
        //         let i = que.pop_front().unwrap() as usize;
        //         max += i;
        //         max = max.min(n);
        //         for j in (i + 1..=max).rev() {
        //             if dp[j - i] { dp[j] = true; }
        //         }
        //         dp[i] = true;
        //     }
        //     if let Some((i, _)) = dp.iter().enumerate().find(|&x| !(*x.1)) {
        //         que.push_back(i as i32);
        //         answer += 1;
        //     }
        // }
        // answer

        // 方法2
        // 要利用有序的特点，来保证之前当前数字nums[i]之前的数字应该都是被覆盖掉的
        // 例如, [1, 3]，
        // 1： 这里最远只能到1，max = 1
        // 3:  这里 nums[i] - max > 1，也就是说不能从nums[i]开始，只能从max+1开始
        //     2： 这里最远能够覆盖到 2 + max = 3, 补充 + 1
        // 3:  这里max >= nums[i]，最远能够覆盖到3 + max = 6，已经大于等于n，结束
        // 需要注意的是max + nums[i]可能会overflow，这里Rust有saturating_add可以应对
        // Passed 0ms 2mb
        let mut que: std::collections::VecDeque<i32> = nums.into_iter().map(|x| x).collect();
        let mut answer = 0;
        let mut max = 0;
        while max < n {
            if let Some(x) = que.pop_front() {
                if x - max > 1 {
                    que.push_front(x);
                    que.push_front(max + 1);
                    answer += 1;
                } else {
                    max = max.saturating_add(x);
                }
            } else {
                que.push_back(max + 1);
                answer += 1;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_patches(vec![], 5), 3);
    assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
    assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28); // 就是它，让我的dp没有过！
}