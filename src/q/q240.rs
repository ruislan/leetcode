use crate::q::Solution;

/// 无rust提交，用kotlin或者Java代替
#[allow(unused)]
impl Solution {
    // 方法与74重名，更改为search_matrix_ii
    pub fn search_matrix_ii(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 方法1
        // 直接迭代matrix，因为有序，内部vec采用二分查找，存在target即返回true
        // 迭代完返回false
        // AC 4ms 2.5mb 129/129
        // matrix.iter().find(|&x| x.binary_search(&target).is_ok()).is_some()

        // 方法2
        // 从row = 0, column = matrix[0].len() - 1位置开始查找（第一行，最后一列）
        // 如果当前位置的数字小于target，说明这一行都小于target，那么row +=1，（下一行，最后一列）
        // 如果当前位置的数字大于target，说明这一列都大于target，那么column -= 1，（这一行，倒数第二列）
        // 如果当前位置的数字等于target，说明已经找到，返回true
        // 循环上述三行，直到row = matrix.len() - 1，column = 0（最后一行，第一列）
        // 都没有找到，说明数字不在其中，返回false
        // 注意rust中column为0时减去1会溢出成最大的那个数字，所以判断条件是column < columns而不是column >= 0
        // AC 4ms 2.7mb
        let (rows, columns) = (matrix.len(), if matrix.is_empty() { 0 } else { matrix[0].len() });
        let (mut row, mut column) = (0, columns - 1);
        while row < rows && column < columns {
            match matrix[row][column].cmp(&target) {
                std::cmp::Ordering::Greater => column -= 1,
                std::cmp::Ordering::Less => row += 1,
                std::cmp::Ordering::Equal => return true,
            }
        }
        false
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::find_number_in2_d_array(vec![vec![]], 1), false);
    assert_eq!(Solution::search_matrix_ii(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 5), true);

    assert_eq!(Solution::search_matrix_ii(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 20), false);

    assert_eq!(Solution::search_matrix_ii(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 30), true);

    assert_eq!(Solution::search_matrix_ii(vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ], 31), false);
}