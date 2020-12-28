use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // 方法1
        // 不能用+，-运算符，那就是要用位运算了
        // 核心点就是每个位的xor就是当前位的数字
        // 每个位的and就是当前位的进位carry
        // 当然要注意的是xor留下的如果是1
        //    那么当前数字 = 1与carry来异或，carry是1与carry做and
        // 否则：
        //    当前数字 = carry，carry就是a和b的当前位做and
        let mut a = a as u32;
        let mut b = b as u32;
        let mut carry = 0;
        let mut answer = 0;
        for i in 0..32 {
            let x_a = a & 1;
            let x_b = b & 1;
            let n = x_a ^ x_b;
            if 0 == n {
                answer = answer | (carry << i);
                carry = x_a & x_b;
            } else {
                answer = answer | ((n ^ carry) << i);
                carry = n & carry;
            }
            a = a >> 1;
            b = b >> 1;
        }
        answer as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_sum(1, 2), 3);
    assert_eq!(Solution::get_sum(-1, 2), 1);
    assert_eq!(Solution::get_sum(-2, 2), 0);
    assert_eq!(Solution::get_sum(-2, 1), -1);
    assert_eq!(Solution::get_sum(i32::MAX, i32::MIN), i32::MAX + i32::MIN);
    assert_eq!(Solution::get_sum(i32::MAX, i32::MAX), (i32::MAX).overflowing_add(i32::MAX).0);
    assert_eq!(Solution::get_sum(i32::MIN, i32::MIN), (i32::MIN).overflowing_add(i32::MIN).0);
}