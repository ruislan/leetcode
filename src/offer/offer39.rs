use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // 方法1
        // 创建一个hashmap，然后将nums[i]放入map
        // 迭代map，当值大于nums.len() / 2 时，即为结果
        // 8ms 2.3mb
        // let mut map = std::collections::HashMap::new();
        // let half = nums.len() / 2;
        // nums.into_iter().for_each(|x| {
        //     let count = map.entry(x).or_insert(0);
        //     *count += 1;
        // });
        // map.into_iter().find(|(k, v)| *v > half).unwrap_or((0, 0)).0

        // 方法2
        // 排序，取中间那个
        // Passed 0ms 2.1mb
        // let mut nums = nums;
        // nums.sort_unstable();
        // nums[nums.len() / 2]

        // 方法3
        // 首先将最大值设定为第一，如果后面那个数字和他相同，则count+1，如果后面那个数字和他不同，则count-1
        // 如果数字为0，则换掉当前数字，因为数字总数过半，那么总有一个数字count会不到0，那个数字就是过半值
        let (mut max, mut count) = (0, 0);
        nums.iter().for_each(|&x| {
            if count == 0 { max = x; }
            if max == x { count += 1; } else { count -= 1; }
        });
        max
    }
}

#[test]
fn test() {
    // 输入非空，且总是存在过半元素
    assert_eq!(Solution::majority_element(vec![1]), 1);
    assert_eq!(Solution::majority_element(vec![1, 2, 3, 2, 2, 2, 5, 4, 2]), 2);
}

