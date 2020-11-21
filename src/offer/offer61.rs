use crate::offer::Solution;

impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        // 方法1
        // 首先排序
        // 然后迭代数字，
        // 虽然最多2个0，否则不符合扑克逻辑，保险起见，我们也可以单独统计0
        // 检查当前数字和前一个数字之间的差值 再减去 1：
        //    差值为负数：则说明前后两个数字一样，返回false
        //    否则：增加空位
        // 返回空位 是否 小于 零的个数
        // Passed 0ms 2mb 时间 O(nlogn)
        // let mut nums = nums;
        // nums.sort_unstable();
        // let zeros = nums.iter().filter(|&&n| n == 0).count() as i32;
        // let mut gap = 0;
        // for i in 1..nums.len() {
        //     if nums[i - 1] != 0 {
        //         if nums[i] == nums[i - 1] { return false; }
        //         gap += nums[i] - nums[i - 1] - 1;
        //     }
        // }
        // gap <= zeros

        // 方法2
        // 找出最小值min和最大值max，如果max与min的差值比4的数量大，那么返回false，否则true
        // Passed 0ms 1.9mb
        let mut sets = std::collections::HashSet::new();
        for x in nums {
            if x != 0 && !sets.insert(x) { return false; }
        }
        let min = *sets.iter().min().unwrap();
        let max = sets.into_iter().max().unwrap();
        max - min < 5
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_straight(vec![1, 2, 3, 4, 5]), true);
    assert_eq!(Solution::is_straight(vec![0, 0, 1, 2, 5]), true);
    assert_eq!(Solution::is_straight(vec![1, 0, 3, 0, 5]), true);
    assert_eq!(Solution::is_straight(vec![1, 1, 3, 0, 5]), false);
    assert_eq!(Solution::is_straight(vec![1, 3, 2, 3, 0]), false);
    assert_eq!(Solution::is_straight(vec![0, 3, 0, 3, 0]), false);
    assert_eq!(Solution::is_straight(vec![0, 1, 0, 3, 0]), true);
    assert_eq!(Solution::is_straight(vec![0, 1, 2, 5, 0]), true);
}