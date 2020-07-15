use crate::Solution;

impl Solution {
    pub fn game(guess: Vec<i32>, answer: Vec<i32>) -> i32 {
        // 方法1
        // zip比对两组数组的对位位置，如果两个数字相同，则计数+1
        // 此题没有Rust的提交解法，所以这里解了之后，用Java改一次提交看结果
        guess.iter().zip(answer.iter()).filter(|(&a, &b)| a == b).count() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::game(vec![], vec![]), 0);
    assert_eq!(Solution::game(vec![1, 2, 3], vec![1, 2, 3]), 3);
    assert_eq!(Solution::game(vec![2, 2, 3], vec![3, 2, 1]), 1);
    assert_eq!(Solution::game(vec![1, 1, 1], vec![2, 2, 2]), 0);
}