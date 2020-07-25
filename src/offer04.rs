use crate::Solution;

/// 同q240
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 方法1
        // 直接迭代matrix，因为有序，内部vec采用二分查找，存在target即返回true
        // 迭代完返回false
        // Passed 4ms
        // matrix.iter().find(|row| row.binary_search(&target).is_ok()).is_some()

        // 方法2
        // 迭代matrix，分为row和column，初始化row = 0，column = columns.len() - 1
        //   1.如果matrix[row][column] < target，说明不在这一行（最大的都比目标小），令row += 1
        //   2.如果matrix[row][column] > target，说明可能在这一行，令column -= 1;
        // 循环1-2直到row = rows.len() - 1和column = 0都不是的情况，则说明找不到
        // Passed 0ms
        let (n, m) = (matrix.len(), if matrix.is_empty() { 0 } else { matrix[0].len() });
        let mut row: usize = 0;
        let mut column: usize = m - 1;
        while column >= 0 && row < n && column < m {
            if matrix[row][column] < target { row += 1; }
            else if matrix[row][column] > target { column -= 1; }
            else { return true; }
        }
        false
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::find_number_in2_d_array(vec![vec![]], 1), false);
    assert_eq!(Solution::find_number_in2_d_array(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 5), true);

    assert_eq!(Solution::find_number_in2_d_array(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 20), false);

    assert_eq!(Solution::find_number_in2_d_array(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 30), true);

    assert_eq!(Solution::find_number_in2_d_array(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 31), false);
}