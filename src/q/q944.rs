use crate::q::Solution;

impl Solution {
    pub fn min_deletion_size(a: Vec<String>) -> i32 {
        let mutex: Vec<Vec<u8>> = a.iter().map(|x| x.bytes().collect::<Vec<u8>>()).collect();
        let mut count = 0;

        for i in 0..mutex[0].len() {
            for j in 1..a.len() {
                if mutex[j][i] < mutex[j - 1][i] {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}