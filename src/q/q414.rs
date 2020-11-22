use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max = vec![None; 3];
        for n in nums {
            if max[0] == None {
                max[0] = Some(n);
            } else if max[0] != None && n >= max[0].unwrap() {
                if n != max[0].unwrap() {
                    max[2] = max[1];
                    max[1] = max[0];
                    max[0] = Some(n);
                }
            } else if max[1] == None {
                max[1] = Some(n);
            } else if max[1] != None && n >= max[1].unwrap() {
                if n != max[1].unwrap() {
                    max[2] = max[1];
                    max[1] = Some(n);
                }
            } else if max[2] == None {
                max[2] = Some(n);
            } else if max[2] != None && n >= max[2].unwrap() {
                max[2] = Some(n);
            }
        }
        if max[2] == None {
            max[0].unwrap()
        } else {
            max[2].unwrap()
        }
    }
}