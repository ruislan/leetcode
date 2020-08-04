use crate::offer::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // 线性查找到第一个数组索引与值不相等的那个数的索引就是差的数
        // 利用库函数find或者循环都可以解决
        // nums.iter().enumerate().find(|&(i, &x)| i as i32 != x).map_or(nums.len(), |(i, &x)| i) as i32

        // 方法2
        // 既然数组是有序的，那么二分查找就很有必要了
        // 令le = 0, ri = nums.len() - 1,迭代当le <= ri时
        // 算出中间值 m = (le + ri) / 2
        // 如果中间值nums[m] == m，说明差的这个数在右侧，令le = m + 1,继续循环
        // 如果中间值nums[m] != m，说明差的这个数在左侧，令ri = m - 1，继续循环
        // 直到le > ri循环结束，nums[le]就是那个第一个开始错的值，
        // 直接返回le即可
        // 需要注意的是rust的数组索引必须是usize，如果输入是空数组，那么ri会出现overflow的错误
        // 所以我们令le和ri都是i32类型，然后在取值的时候nums[m]的m再转化为usize类型，这个时候我们是能够保证m一定是usize的
        // 因为ri如果为负值，是进不了循环的，也就不存在计算m和取值的问题
        let (mut le, mut ri) = (0, nums.len() as i32 - 1);
        while le <= ri {
            let m = (le + ri) / 2;
            if nums[m as usize] == m { le = m + 1; } else { ri = m - 1; }
        }
        le
    }
}

#[test]
fn test() {
    assert_eq!(Solution::missing_number(vec![]), 0);
    assert_eq!(Solution::missing_number(vec![1]), 0);
    assert_eq!(Solution::missing_number(vec![0]), 1);
    assert_eq!(Solution::missing_number(vec![0, 1, 2]), 3);
    assert_eq!(Solution::missing_number(vec![0, 1, 3]), 2);
    assert_eq!(Solution::missing_number(vec![0, 1, 2, 3, 4, 5, 6, 7, 9]), 8);
}