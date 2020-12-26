use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        // 方法1
        // 记录每个matrix[row][col]的向上最大的高度h
        // 迭代matrix：
        //     设置矩阵面积为area，当前高度h,宽度w,
        //     matrix[row][col]向左搜索,每次搜索w = w + 1
        //     h 取较小的那个，area = max(area, h * w)
        // Passed 8ms 4.8mb
        if matrix.is_empty() || matrix[0].is_empty() { return 0; }

        let mut rows = matrix.len();
        let mut cols = matrix[0].len();
        let mut heights = vec![vec![0; cols]; rows];
        let mut area = 0;
        for row in 0..rows {
            for col in 0..cols {
                if matrix[row][col] == '1' {
                    heights[row][col] = if row > 0 { heights[row - 1][col] + 1 } else { 1 };
                    let mut h = rows;
                    let mut w = 1;
                    for i in (0..=col).rev() {
                        if heights[row][i] == 0 { break; }
                        h = h.min(heights[row][i]);
                        area = area.max(h * w);
                        w += 1;
                    }
                }
            }
        }

        area as i32
    }
}