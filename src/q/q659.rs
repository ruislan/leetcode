use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        // 方法1
        // 首先用hashmap统计频率到freq中
        // 设置一个名为ends的hashmap作为记录数字nums[i]作为子序列结尾的次数
        // 迭代x in nums:
        //    如果freq[x]还有：
        //       检查x的前一个数字是否是结尾，来看我们是否能够加入到前一个序列中，
        //       也即是ends[x - 1] > 0：
        //          如果能加入：
        //             freq[x] -= 1，当前数字的频率少1
        //             ends[x - 1] -= 1， 前一个数字不再作为这个子序列的结尾
        //             ends[x] += 1，当前数字作为这个子序列新的结尾
        //          如果不能加入：
        //             那么就要新开一个子序列，
        //             freq[x+1]和freq[x+2]都必须大于0：
        //                freq[x],freq[x+1],freq[x+2]各自的频率少1
        //                ends[x+2] += 1，x+2作为这个新子序列的结尾
        //             否则：无法新开一个满足条件的子序列，返回false
        // 检查完毕，都符合，返回true
        // Passed 20ms 2.1mb
        let mut freq = std::collections::HashMap::new();
        let mut ends = std::collections::HashMap::new();
        for i in 0..nums.len() {
            *freq.entry(nums[i]).or_insert(0) += 1;
        }
        for x in nums {
            let freq_x = *freq.get(&x).unwrap_or(&0);
            if freq_x > 0 {
                let end = *ends.get(&(x - 1)).unwrap_or(&0);
                if end > 0 {
                    if let Some(it) = freq.get_mut(&x) { *it -= 1; }
                    if let Some(it) = ends.get_mut(&(x - 1)) { *it -= 1; }
                    *ends.entry(x).or_insert(0) += 1;
                } else {
                    if *freq.get(&(x + 1)).unwrap_or(&0) > 0 &&
                        *freq.get(&(x + 2)).unwrap_or(&0) > 0 {
                        if let Some(it) = freq.get_mut(&x) { *it -= 1; }
                        if let Some(it) = freq.get_mut(&(x + 1)) { *it -= 1; }
                        if let Some(it) = freq.get_mut(&(x + 2)) { *it -= 1; }
                        *ends.entry(x + 2).or_insert(0) += 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_possible(vec![1]), false);
    assert_eq!(Solution::is_possible(vec![1, 2]), false);
    assert_eq!(Solution::is_possible(vec![1, 2, 3]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 4, 4, 5]), false);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5]), true);
    assert_eq!(Solution::is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5]), true);
}