use crate::q::Solution;

impl Solution {
    // q189的方法名是'rotate'，这里重复了，所以这里重命名为rotate_matrix
    pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
        // 方法1
        // 矩阵为空，不需要操作
        // 设置两个变量begin=0和end=n-1，分别表示横竖的起点和终点，有
        // a a a b
        // d ... b
        // d ... b
        // d c c c
        // 旋转意味着，
        // 当n = 1时（begin >= end），不需要旋转，因为n * n 的矩阵，只有1个数是不需要旋转的
        // 当n > 1时，
        // a b c d 顺时针变成 ~d a ~b c
        // 如果不能使用额外的空间来进行交换，那么可以采用
        // a与b交换，a[b]与c交换（反向），a[c]与d交换（反向）即完成了所有的交换，
        // 每转换一次，begin += 1, end -= 1
        // 如果begin >= end，就不需要再操作了
        // Passed 0ms 2mb
        // let (mut begin, mut end) = (0, matrix.len().saturating_sub(1));
        // while begin < end {
        //     (begin..end).for_each(|i| {
        //         // a 与 b 交换
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[i][end];
        //         matrix[i][end] = matrix[begin][i] ^ matrix[i][end];
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[i][end];
        //         // a(现b)与c交换（反向）
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[end][end - i + begin];
        //         matrix[end][end - i + begin] = matrix[begin][i] ^ matrix[end][end - i + begin];
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[end][end - i + begin];
        //         // a(现c)与d交换（反向）
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[end - i + begin][begin];
        //         matrix[end - i + begin][begin] = matrix[begin][i] ^ matrix[end - i + begin][begin];
        //         matrix[begin][i] = matrix[begin][i] ^ matrix[end - i + begin][begin];
        //     });
        //     begin += 1;
        //     end -= 1;
        // }

        // 方法2
        // 沿着左右对角线旋转，再上下中线旋转即可
        // Passed 0ms 2mb
        (0..matrix.len()).for_each(|i| {
            (i + 1..matrix.len()).for_each(|j| {
                matrix[i][j] = matrix[i][j] ^ matrix[j][i];
                matrix[j][i] = matrix[i][j] ^ matrix[j][i];
                matrix[i][j] = matrix[i][j] ^ matrix[j][i];
            });
        });
        matrix.iter_mut().for_each(|col| col.reverse());
    }
}

#[test]
fn test() {
    let mut mat = vec![vec![]];
    Solution::rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![]]);

    mat = vec![vec![1]];
    Solution::rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![1]]);

    mat = vec![vec![1, 2], vec![1, 2]];
    Solution::rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![1, 1], vec![2, 2]]);

    mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

    mat = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
    Solution::rotate_matrix(&mut mat);
    assert_eq!(mat, vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]]);
}