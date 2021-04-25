use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 暴力穷举
        // 范围是[1,50K]，测试了几个50K的数据，
        // 都在1s内解决，但是这种解法，肯定会超时
        // 因为leetcode应该算的是所有测试的整体时间
        // 超时
        // let n = nums.len();
        // let mut answer = 0;
        // for i in 0..n {
        //     let mut odds = 0;
        //     for j in i..n {
        //         if nums[j] & 1 == 1 {
        //             odds += 1;
        //         }
        //         if odds == k {
        //             answer += 1;
        //         }
        //         if odds > k {
        //             break;
        //         }
        //     }
        // }
        // answer

        // 方法2
        // 双指针
        // 我们观察一下范例 2,2,2,1,2,2,1,2,2,2 k=2
        // 如果要包含两个Odd，那么必然会有1,2,2,1这个子数组
        // 然后我们看到左边有3个元素2,2,2，右边有3个元素2,2,2
        // 那么我们计算左边的3个加上这个必然有的就是4个，
        // 同样的，右边也是如此就是4个
        // 而最终的结果就是4 * 4，就是16个，这和范例的结果相同
        // 说明此方式可行
        // 那么，我们就先找到刚好符合条件的子数组，这个子数组肯定包含了左边，但是不包含右边
        // 所以我们继续将右边扩展直到不符合的前一个，这样我们就统计出了右边的个数
        // 然后我们再收缩左边的指针，直到不符合的前一个，这样我们就统计出了左边的个数
        // 然后按照刚刚的方式，(左边+1)*(右边+1)就等于结果
        // 此时的左边再移动一位，就少一个odd，而右边再移动一位，就多一个odd
        // 所以，我们处理完成之后，移动左边的指针，然后让odd减去1，然后继续。
        // AC 16ms 2.5mb
        let n = nums.len();
        let mut answer = 0;
        let mut lo = 0;
        let mut hi = 0;
        let mut odd_count = 0;
        while hi < n {
            if nums[hi] & 1 == 1 {
                odd_count += 1;
            }
            if odd_count == k {
                let mut left_count = 1;
                while lo < n && nums[lo] & 1 == 0 {
                    left_count += 1;
                    lo += 1;
                }
                let mut right_count = 1;
                while hi + 1 < n && nums[hi + 1] & 1 == 0 {
                    hi += 1;
                    right_count += 1;
                }
                answer += left_count * right_count;

                lo += 1;
                odd_count -= 1;
            }
            if odd_count < k {
                hi += 1;
            }             
        }
        answer
    }
}
