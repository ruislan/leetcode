use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort();
        for i in (0..a.len() - 2).rev() {
            if a[i] + a[i + 1] > a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }
}