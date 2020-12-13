use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        // 方法1
        // 暴力穷举
        // 将所有的数字组合都尝试一遍
        // 时间：O(n^2)，空间：O(1)
        // Passed 424ms 2.1mb
        // let k = k as usize;
        // let n = nums.len();
        // for i in 0..n {
        //     for j in i + 1..n {
        //         if (nums[i] as i64 - nums[j] as i64).abs() <= t as i64 && j - i <= k {
        //             return true;
        //         }
        //     }
        // }
        // false

        // 方法2
        // 方法1每次都找了接近n的大小
        // 其实不需要这么找超过k长度的，因为条件1就是|j - i| <= k
        // 所以维持一个一个k长度的窗口就行了，这样减少探索量，
        // 不过提高效率有限，非指数级
        // 时间：O(n^2)，空间：O(1)
        // Passed 284ms 2mb
        // let n = nums.len();
        // let k = k as usize;
        // for i in 0..n {
        //     for j in i + 1..(i + k + 1).min(n) {
        //         if (nums[i] as i64 - nums[j] as i64).abs() <= t as i64 {
        //             return true;
        //         }
        //     }
        // }
        // false

        // 方法3
        // 继续优化
        // 假设我们当前索引i到i + k的所有数字是有序的话
        // 那么我们就可以使用二分搜索来快速找到这样一个数字它满足
        // |nums[i] - nums[j]| <= t
        // 也就是这个数字nums[i]的前后t范围内至少找得到1个数字
        // upper_bound = nums[i] + t
        // lower_bound = nums[i] - t
        // 这样查找就从O(n)变成了O(logn)，然后整个时间就成了O(nlogn)
        // 如果要快速用二分查找，这里用到btreeset这个数据结构
        // 这个btreeset的结构维持一个k大小的长度
        // 我们查找一个k长度的数组的上下界的数字,只要找得到这个数字就能表示满足条件，返回即可
        // Passed 12ms 2.5mb
        let n = nums.len();
        let k = k as usize;
        let t = t as i64;
        let mut set = std::collections::BTreeSet::new();
        for i in 0..n {
            let upper_bound = nums[i] as i64 + t;
            let lower_bound = nums[i] as i64 - t;
            if set.range(lower_bound..=upper_bound).count() > 0 {
                return true;
            }
            set.insert(nums[i] as i64);
            if set.len() > k {
                set.remove(&(nums[i - k] as i64));
            }
        }
        false
    }
}

#[test]
fn test() {
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3), false);
    assert_eq!(Solution::contains_nearby_almost_duplicate(vec![-2147483648, 2147483647], 1, 1), false);
}