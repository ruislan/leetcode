use crate::q::Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        // 方法1
        // let mut carry = 1;
        // let mut nums: Vec<i32> = Vec::new();
        // loop {
        //     let num = digits.pop();
        //     if num == None { break; }
        //     let new_num = num.unwrap() + carry;
        //     carry = new_num / 10;
        //     nums.insert(0, new_num % 10);
        // }
        // if carry > 0 { nums.insert(0, carry); }
        // nums

        // 方法2
        let (mut carry, len) = (1, digits.len());
        for i in 0..len {
            let _i = len - 1 - i;
            let num = digits[_i] + carry;
            carry = num / 10;
            digits[_i] = num % 10;
        }
        if carry > 0 { digits.insert(0, carry); }
        digits
    }
}