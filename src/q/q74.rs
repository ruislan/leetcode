use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 方法1
        // 朴素的方法就是一个一个的查，很不高效O(r * c)
        // 方法2
        // 贪心+二分查找，找出刚好比target大的那个row
        // 再二分查找row，最坏O(r * logc)
        // AC 0ms 2mb
        let mut rows = matrix.len();
        let mut cols = matrix[0].len();
        for row in 0..rows {
            if matrix[row][cols - 1] >= target {
                match matrix[row].binary_search(&target) {
                    Ok(_) => return true,
                    Err(_) => return false,
                }
            }
        }
        false
    }
}