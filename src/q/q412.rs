use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for i in 1..=n {
            if i % 3 == 0 && i % 5 != 0 {
                res.push(String::from("Fizz"));
            } else if i % 5 == 0 && i % 3 != 0 {
                res.push(String::from("Buzz"))
            } else if i % 5 == 0 && i % 3 == 0 {
                res.push(String::from("FizzBuzz"));
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}