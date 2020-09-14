use crate::q::Solution;

impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 { return false; }
        let mut top = None;
        let mut up = 0;
        let mut down = 0;
        for i in 1..a.len() {
            if top == None {
                if a[i - 1] < a[i] {
                    up += 1;
                } else if a[i - 1] > a[i] {
                    top = Some(i - 1);
                    down += 1;
                } else {
                    return false;
                }
            } else {
                if up == 0 { return false; }
                if a[i - 1] > a[i] {
                    down += 1;
                } else {
                    return false;
                }
            }
        }
        if top == None || up == 0 || down == 0 { return false; }
        return true;
    }
}