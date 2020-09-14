use crate::q::Solution;

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        // 方法1，用一个初始值为-1的变量max来存储右边最大的值，从后向前迭代arr，遇到max被替换之前，替换当前的值为max
        // Passed 0ms 2.1mb (4ms 2.1mb)
        // let mut arr = arr;
        // let mut max = -1;
        // for i in (0..arr.len()).rev() {
        //     if arr[i] > max {
        //         arr[i] ^= max;
        //         max ^= arr[i];
        //         arr[i] ^= max;
        //     } else {
        //         arr[i] = max;
        //     }
        // }
        // arr

        // 方法2，初始化一个数组res，初始值为-1，长度和arr一样，从后向前迭代arr，从len - 2开始，比较res与arr的i + 1位置，把大的放入res[i]
        // Passed 4ms 2.1mb
        let mut res = vec![-1; arr.len()];
        for i in (0..arr.len() - 1).rev() {
            res[i] = arr[i + 1].max(res[i + 1]);
        }
        res
    }
}

#[test]
fn test_q1299() {
    assert_eq!(Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]), vec![18, 6, 6, 6, 1, -1]);
}