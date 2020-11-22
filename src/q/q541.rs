use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let k = k as usize;
        let mut s = s;
        let phase = s.len() / k;
        if phase < 1 { return s.get_mut(..).unwrap().chars().rev().collect::<String>(); }

        let mut res = String::new();
        let mut forward = false;
        for i in 0..phase {
            if i % 2 == 0 {
                res.push_str(&s.get_mut(i * k..(i + 1) * k).unwrap().chars().rev().collect::<String>());
                forward = false;
            } else {
                res.push_str(s.get(i * k..(i + 1) * k).unwrap());
                forward = true;
            }
        }
        if !forward {
            res.push_str(s.get(phase * k..).unwrap());
        } else {
            res.push_str(&s.get(phase * k..).unwrap().chars().rev().collect::<String>());
        }
        res
    }
}