use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut v = vec![a, b, c];
        v.sort();
        let (x, y, z) = (v[0], v[1], v[2]);
        vec![
            if z - x == 2 { 0 } else if y - x <= 2 || z - y <= 2 { 1 } else { 2 },
            z - x - 2
        ]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
    assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
}