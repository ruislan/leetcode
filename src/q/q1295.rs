use crate::q::Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        // 方法一：迭代每位数字，数字每位除以10，在等于0的时候计算位数，位数是偶数的留下，再进行统计
        // Passed 0ms 2.1mb
        // nums.iter().filter(|&&x| {
        //     let mut even = true;
        //     let mut x = x;
        //     while x > 0 {
        //         x /= 10;
        //         even = !even;
        //     }
        //     even
        // }).count() as i32

        // 方法二：迭代每位数字，转换成字符串，计算字符串的长度是否是偶数，并统计数量
        // 思考：数字换成字符串的性能真的优于方法一的朴素计算方式吗？
        // Passed 0ms 2.1mb
        // nums.iter().filter(|x| { x.to_string().len() % 2 == 0 }).count() as i32

        // 方法三：通过数学的log10位底来计算数字，公式为log(num) + 1即为位数，并进行统计
        // Passed 0ms 2.5mb
        nums.iter().filter(|&&x| ((x as f32).log10() as i32 + 1) % 2 == 0).count() as i32
    }
}

#[test]
fn test_q1295() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
    assert_eq!(Solution::find_numbers(vec![555]), 0);
    assert_eq!(Solution::find_numbers(vec![100000]), 1);
    assert_eq!(Solution::find_numbers(vec![1]), 0);
}