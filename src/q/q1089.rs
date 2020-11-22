use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut len = 0;
        let mut a_len = 0;
        for i in 0..arr.len() {
            if len >= arr.len() { break; }
            len += 1;
            a_len += 1;
            if arr[i] == 0 { len += 1; }
        }
        if len > arr.len() {
            len -= 2;
            a_len -= 1;
            arr[len] = 0;
        }
        for i in (0..a_len).rev() {
            len -= 1;
            arr[len] = arr[i];
            if arr[i] == 0 {
                len -= 1;
                arr[len] = 0;
            }
        }
    }
}