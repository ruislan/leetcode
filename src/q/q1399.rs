use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        // 方法1
        // 数位和可以把数字换成str再分离成各个字符再转换成数字，然后相加得到数位和，存储到1-36的数组中
        // 然后统计map中,值最大的数的个数即可
        let mut arr = vec![0; 37];
        (1..=n).for_each(|mut x| {
            // let sum: u32 = x.to_string().chars().map(|ch| ch.to_digit(10).unwrap_or(0)).sum();
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            arr[sum as usize] += 1;
        });
        let max = *arr.iter().max().unwrap();
        arr.iter().filter(|&&n| n == max).count() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_largest_group(13), 4);
    assert_eq!(Solution::count_largest_group(2), 2);
    assert_eq!(Solution::count_largest_group(15), 6);
    assert_eq!(Solution::count_largest_group(24), 5);
    assert_eq!(Solution::count_largest_group(10000), 1);
}