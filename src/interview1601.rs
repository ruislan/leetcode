use crate::Solution;

impl Solution {
    pub fn swap_numbers(numbers: Vec<i32>) -> Vec<i32> {
        // 方法一，xor操作三次
        // let mut numbers = numbers;
        // numbers[0] = numbers[0] ^ numbers[1];
        // numbers[1] = numbers[0] ^ numbers[1];
        // numbers[0] = numbers[0] ^ numbers[1];
        // numbers

        // 方法二，先求出差值，替换n[0]，然后n[1]减去这个差值存储到n[1]，然后n[0] + n[1]存储到n[0]
        let mut numbers = numbers;
        numbers[0] = numbers[1] - numbers[0];
        numbers[1] = numbers[1] - numbers[0];
        numbers[0] = numbers[0] + numbers[1];
        numbers
    }
}

#[test]
fn test_interview_16_01() {
    assert_eq!(vec![1, 3], Solution::swap_numbers(vec![3, 1]));
    assert_eq!(vec![1, -3], Solution::swap_numbers(vec![-3, 1]));
}