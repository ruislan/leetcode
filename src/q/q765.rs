use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        // 方法1
        // 看了一下取值范围，只有[4,60]
        // 也就是说用暴力解决O(n^2)最坏也才3600次迭代
        // 能暴力就不哔哔
        // 简单来说就是找出后面是跟你一对的数字之后，
        // 与你下一个数字交换，如果找不到，那么说明你下一个数字就是和你一对的
        // 然后跳过2个数字，重复直到数组结束
        // 感觉成了简单题了，所以这题可能考察的不是这样的解题方式，这就是出题出失败了呀
        // 有时间就思考一下其他解法吧
        // AC 0ms 1.9mb
        let n = row.len();
        let mut row = row;
        let mut i = 0;
        let mut answer = 0;
        while i < n - 2 {
            let mate = if row[i] % 2 == 0 { row[i] + 1 } else { row[i] - 1 };
            for j in i + 2..n {
                if row[j] == mate {
                    row.swap(j, i + 1);
                    answer += 1;
                    break;
                }
            }
            i += 2;
        }
        answer
    }
}