use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 创建二维数组，将距离依次算出来，然后按照距离二维数组
        // Passed 12ms 2.9mb
        let mut metrix = Vec::new();
        (0..r).for_each(|row| (0..c).for_each(|col| metrix.push(((row - r0).abs() + (col - c0).abs(), vec![row, col]))));
        metrix.sort_unstable_by_key(|(k, v)| *k);
        metrix.into_iter().map(|x| x.1).collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::all_cells_dist_order(1, 2, 0, 0), vec![vec![0, 0], vec![0, 1]]);
    assert_eq!(Solution::all_cells_dist_order(2, 2, 0, 1), vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]]);
    assert_eq!(Solution::all_cells_dist_order(2, 3, 1, 2), vec![vec![1, 2], vec![0, 2], vec![1, 1], vec![0, 1], vec![1, 0], vec![0, 0]]);
}