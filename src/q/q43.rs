use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // 方法1
        // 将num1和Num2的每个数字相乘
        // 注意num1[i] * num2[j]乘出来的结果的位数要向左移动
        // 最后将所有的结果加起来就是最后的结果
        // O(n^2 + n), n=110，应该可以接受
        // AC 4ms 2.3mb
        let mut values = Vec::new();
        let mut max = 0;
        for (j, digit2) in num2.chars().rev().enumerate() { // 456
            let x2 = digit2.to_digit(10).unwrap();
            let mut carry = 0;
            let mut value = vec![0; j];
            for (i, digit1) in num1.chars().rev().enumerate() {
                let x1 = digit1.to_digit(10).unwrap();
                let mut y = x1 * x2 + carry;
                carry = y / 10;
                y = y % 10;
                value.push(y);
            }
            if carry > 0 { value.push(carry); }
            max = max.max(value.len());
            values.push(value);
        }

        let mut answer = Vec::new();
        let mut carry = 0;
        for i in 0..max {
            let mut x = 0;
            for value in values.iter() {
                if value.len() > i {
                    x += value[i];
                }
            }
            x += carry;
            carry = x / 10;
            x = x % 10;
            answer.push(x);
        }
        if carry > 0 {
            answer.push(carry);
        }
        // check zero
        if answer.iter().filter(|&&x| x > 0).count() > 0 {
            answer.into_iter().rev().map(|x| x.to_string()).collect()
        } else {
            String::from("0")
        }
    }
}