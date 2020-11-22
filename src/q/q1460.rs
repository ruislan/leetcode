use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        // 方法1
        // 将target和arr排序，然后返回两个数组是否相等
        // let (mut target, mut arr) = (target, arr);
        // target.sort();
        // arr.sort();
        // target == arr

        // 方法2
        // 构建长度为1001的数组res，将target元素作为下标设置res[i]的值为1
        // 迭代arr，将arr[i]作为下标，如果res[arr[i]]值不为1，返回false
        // 返回true
        let mut res = vec![0; 1001];
        target.iter().for_each(|&x| res[x as usize] += 1);
        for x in arr {
            if res[x as usize] == 0 { return false; }
            res[x as usize] -= 1;
        }
        true
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::can_be_equal(vec![], vec![]), false);
    assert_eq!(Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]), true);
    assert_eq!(Solution::can_be_equal(vec![7], vec![7]), true);
    assert_eq!(Solution::can_be_equal(vec![1, 12], vec![12, 1]), true);
    assert_eq!(Solution::can_be_equal(vec![3, 7, 9], vec![3, 7, 11]), false);
    assert_eq!(Solution::can_be_equal(vec![1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1]), true);
}