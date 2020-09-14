use crate::q::Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if &a == &b { return -1; }
        if &a.len() > &b.len() { a.len() as i32 } else { b.len() as i32 }
    }
}