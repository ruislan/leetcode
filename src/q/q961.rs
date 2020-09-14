use crate::q::Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut bag = vec![0; 10000];
        for i in 0..a.len() {
            bag[a[i] as usize] += 1;
        }
        for i in 0..bag.len() {
            if bag[i] >= a.len() / 2 {
                return i as i32;
            }
        }
        return 0;
    }
}