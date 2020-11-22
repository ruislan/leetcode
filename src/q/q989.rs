use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn add_to_array_form(mut a: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut carry = 0i32;
        let mut res = Vec::new();
        loop {
            if a.is_empty() && k == 0 && carry == 0 { break; }
            if let Some(n) = a.pop() { carry += n; }
            carry += k % 10;
            res.push(carry % 10);
            k /= 10;
            carry /= 10;
        }
        res.reverse();
        res
    }
}