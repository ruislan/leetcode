use crate::q::Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        // 方法一，数字除以10得到每位数字，算出每位数字的乘积和和，然后再相减
        // 注意i位上可能是0，如果是0，就是负数
        // passed 0ms;2mb
        let mut n = n;
        let (mut multiply, mut sum) = (1, 0);
        while n > 0 {
            let num = n % 10;
            multiply *= num;
            sum += num;
            n /= 10;
        }
        multiply - sum
    }
}

#[test]
fn test_q1281() {
    assert_eq!(15, Solution::subtract_product_and_sum(234));
    assert_eq!(21, Solution::subtract_product_and_sum(4421));
    assert_eq!(-1, Solution::subtract_product_and_sum(100000));
    assert_eq!(59004, Solution::subtract_product_and_sum(99999));
    assert_eq!(0, Solution::subtract_product_and_sum(1));
}