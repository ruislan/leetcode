use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // 方法1
        // 变量max记录当前最大的子数组之和，sum记录当前子数组的临时和
        // 迭代nums，sum结果和当前数n比较，取大的那个数字为sum，然后sum和max进行比较，取大的那个为max
        // 直到迭代结束，返回max即可
        // 我们来模拟一下这个算法是否正确，-2,1,-3,4,-1,2,1,-5,4
        // max初始值为i32::min，sum为0
        // -2：sum = max(0 + -2, -2) = -2；max = max(i32::min, sum) = -2
        //  1：sum = max(-2 + 1, 1) = 1；max = max(-2, 1) = 1
        // -3：sum = max(-3 + 1, -3) = -2； max = max(1, -2) = 1
        //  4：sum = max(-2 + 4, 4) = 4；max = max(1, 4) = 4
        // -1: sum = max(4 + -1, -1) = 3；max = max(4, 3) = 4
        // 2: sum = max(3 + 2, 2) = 5；max = max(4, 5) = 5
        // 1: sum = max(5 + 1, 1) = 6; max = max(5,6) = 6
        // -5: sum = max(6 + -5, -5) = 1; max = max(6,1) = 6
        // 4: sum = max(1 + 4, 4) = 5;max = max(6,5) = 6
        // 结果为6，且增加一个数都能找到最大的那个子数组的和，算法正确
        let (mut max, mut sum) = (i32::min_value(), 0);
        for n in nums {
            sum = n.max(n + sum);
            max = max.max(sum);
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_sub_array(vec![100]), 100);
    assert_eq!(Solution::max_sub_array(vec![0]), 0);
    assert_eq!(Solution::max_sub_array(vec![-100]), -100);
    assert_eq!(Solution::max_sub_array(vec![-100, -90]), -90);
    assert_eq!(Solution::max_sub_array(vec![-100, -90, 0]), 0);
    assert_eq!(Solution::max_sub_array(vec![-100, -90, 0, 10, 100]), 110);
    assert_eq!(Solution::max_sub_array(vec![1, 10, 100]), 111);
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}