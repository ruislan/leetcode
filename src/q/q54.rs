use crate::q::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        // 方法1
        // 设定四个参数为y0,x0,xMax,yMax分别表示y轴起止点和x轴起止点
        // 循环，当数组长度为yMax - y0 > 1 || xMax - x0 > 1进入循环
        // 因为假设没有数组，不用打印
        // 假设只有1个数组，顺序打印
        // 假设只有2个数组，顺序加逆序打印
        // 假设有3个数组:
        //  顺序打印y0[x0:xMax]，
        //  顺序打印xMax[y1:yMax-1]，
        //  逆序打印yMax[xMax : x0]
        //  逆序打印x0[yMax-1: y1]
        //  然后令y0 += 1, x0 += 1, xMax -= 1, yMax -= 1，这一步相当于去掉最外圈。
        // Passed 0ms 2.1mb
        if matrix.is_empty() || matrix[0].is_empty() { return vec![]; }

        let mut res = Vec::new();
        let (mut row_begin, mut row_end) = (0, matrix.len() - 1);
        let (mut col_begin, mut col_end) = (0, matrix[0].len() - 1);

        while row_begin <= row_end && col_begin <= col_end {
            (col_begin..=col_end).for_each(|i| res.push(matrix[row_begin][i]));
            (row_begin + 1..row_end).for_each(|i| res.push(matrix[i][col_end]));
            if row_begin != row_end { (col_begin..=col_end).rev().for_each(|i| res.push(matrix[row_end][i])); }
            if col_begin != col_end { (row_begin + 1..row_end).rev().for_each(|i| res.push(matrix[i][col_begin])); }

            row_begin += 1;
            row_end = row_end.saturating_sub(1);
            col_begin += 1;
            col_end = col_end.saturating_sub(1);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::spiral_order(vec![]), vec![]);
    assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(Solution::spiral_order(vec![vec![1, 2]]), vec![1, 2]);
    assert_eq!(Solution::spiral_order(vec![vec![1], vec![2]]), vec![1, 2]);
    assert_eq!(Solution::spiral_order(vec![vec![1, 2], vec![3, 4]]), vec![1, 2, 4, 3]);
    assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
}