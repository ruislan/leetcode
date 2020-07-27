use crate::q::Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        // 方法1
        // 排序arr，然后从1..arr.len()开始迭代，如果两个数之间的差值不相同，直接返回false，迭代完返回true
        let mut arr = arr;
        arr.sort_by(|a, b| b.cmp(a));
        let q = arr[0] - arr[1];
        arr.windows(2).all(|x| x[0] - x[1] == q)
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::can_make_arithmetic_progression(vec![]), false);
    assert_eq!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]), true);
    assert_eq!(Solution::can_make_arithmetic_progression(vec![1, 2, 4]), false);
    assert_eq!(Solution::can_make_arithmetic_progression(vec![1, 3]), true);
}