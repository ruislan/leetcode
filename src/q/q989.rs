use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn add_to_array_form(mut a: Vec<i32>, mut k: i32) -> Vec<i32> {
        // 方法1
        // Passed 12ms 2mb
        // let mut carry = 0i32;
        // let mut answer = Vec::new();
        // loop {
        //     if a.is_empty() && k == 0 && carry == 0 { break; }
        //     if let Some(n) = a.pop() { carry += n; }
        //     carry += k % 10;
        //     answer.push(carry % 10);
        //     k /= 10;
        //     carry /= 10;
        // }
        // answer.reverse();
        // answer

        // 方法1的另外一种写法
        let mut k = k;
        let mut b = Vec::new();
        while k > 0 {
            b.push(k % 10);
            k /= 10;
        }

        let mut a = a;
        let mut carry = 0;
        let mut answer = Vec::new();
        let (mut i, mut j) = (0, 0);
        a.reverse();
        while i < a.len() || j < b.len() {
            let x = if i < a.len() { a[i] } else { 0 };
            let y = if j < b.len() { b[i] } else { 0 };
            let sum = x + y + carry;
            carry = sum / 10;
            answer.push(sum % 10);
            i += 1;
            j += 1;
        }
        if carry > 0 { answer.push(carry); }
        answer.reverse();
        answer
    }
}