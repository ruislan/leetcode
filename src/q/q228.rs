use crate::q::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        // 方法1
        // 因为已经有序，且按照升序排列，
        // 创建一个变量ranges来保存range
        // 从1..size进行迭代，如果后面比前面大1，则加入当前的range
        // 否则，建立一个新的range
        // 迭代所有的ranges：
        //    如果range只有1个，组合成"$num"
        //    大于1个，则取第一个和最后一个，中间用"->"连接
        // Passed 0ms 2.2mb
        // if nums.is_empty() { return Vec::new(); }
        //
        // let mut ranges = vec![vec![nums[0]]];
        // let mut range = &mut ranges[0];
        // for i in 1..nums.len() {
        //     if nums[i] - nums[i - 1] == 1 {
        //         range.push(nums[i]);
        //     } else {
        //         ranges.push(vec![nums[i]]);
        //         let la = ranges.len() - 1;
        //         range = &mut ranges[la];
        //     }
        // }
        //
        // ranges.into_iter().map(|range| {
        //     if range.len() == 1 {
        //         format!("{}", range[0])
        //     } else {
        //         format!("{}->{}", range[0], range[range.len() - 1])
        //     }
        // }).collect()

        // 方法2
        // 尝试优化一下方法1，不用存储全部的range，只需要记录start
        // Passed 0ms 2.1mb
        if nums.is_empty() { return Vec::new(); }
        let mut answer = Vec::new();
        let mut base = nums[0];
        for i in 1..=nums.len() {
            if i == nums.len() || nums[i] - nums[i - 1] != 1 {
                if nums[i - 1] - base > 0 {
                    answer.push(format!("{}->{}", base, nums[i - 1]));
                } else {
                    answer.push(format!("{}", base));
                }
                if i < nums.len() { base = nums[i]; }
            }
        }
        answer
    }
}

#[test]
fn test() {
    let empty: Vec<String> = Vec::new();
    assert_eq!(Solution::summary_ranges(vec![]), empty);
    assert_eq!(Solution::summary_ranges(vec![0]), vec!["0".to_string()]);
    assert_eq!(Solution::summary_ranges(vec![-1]), vec!["-1".to_string()]);
    assert_eq!(Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
               vec!["0".to_string(),
                    "2->4".to_string(),
                    "6".to_string(),
                    "8->9".to_string(),
               ]);
    assert_eq!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
               vec!["0->2".to_string(),
                    "4->5".to_string(),
                    "7".to_string(),
               ]);
}