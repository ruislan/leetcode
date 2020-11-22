use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if 0 == num { return String::from("0"); };
        let mut res = String::new();
        let ng = num < 0;
        let mut num = num.abs();
        while num != 0 {
            let curr = num % 7;
            num /= 7;
            res.push_str(&curr.to_string());
        }
        if ng { res.push('-'); }
        res.chars().rev().collect::<String>()
    }
}