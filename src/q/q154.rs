use crate::q::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // 方法1
        // 排序之后取第一个
        // let mut nums = nums;
        // nums.sort_unstable();
        // nums[0]

        // 方法2
        // 线性查找最小值
        // nums.into_iter().min().unwrap()

        // 方法3
        // 因为这是一道“困难”题，所以方法1虽然可以解决，但是级别明显不够
        // 因为数组是升序，按照某个位置进行了旋转（像向右拉动了一段距离）之后，
        // 局部任然是升序，只是在某个点会突然降到最小值，又开始升序，所以右侧端点就特别重要
        // 我们通过不断找出中间端点与右侧端点的关系来不断缩小数组，直到找到那个最小值
        // 分别记左右两个端点为le,re，并计算出中间点mid
        // 如果mid < re，则说明最小值在左侧(包含mid)，
        // 如果mid > re，则说明最小值在右侧(不包含mid)，
        // 因为有可能存在重复值，所以mid == re的情况，不能判断在左边还是右边，
        // 但是因为mid == re，所以不管mid是不是最小点，这个点多一个少一个对结果都无影响，
        // 所以我们可以剔除这个数
        // 因为是最小值，那么有两个最小值，如果不是最小值，那么去掉一个比最小值大的有什么关系呢
        // 也即是说re -= 1即可删除掉re这个点
        // 一直循环到le == re的时候，也即是数组只剩下最后一个数字，那个数字，就是最小值
        let (mut le, mut re) = (0, nums.len() - 1);
        while le < re {
            let mid = (le + re) / 2;
            if nums[mid] < nums[re] { re = mid; } else if nums[mid] > nums[re] { le = mid + 1; } else { re -= 1; }
        }
        nums[le]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_min(vec![1]), 1);
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    assert_eq!(Solution::find_min(vec![5, 3, 1]), 1);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 2, 0]), 0);
    assert_eq!(Solution::find_min(vec![2, 0, 1, 2, 2]), 0);
}