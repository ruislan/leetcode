use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 方法1
        // AC 0ms 1.9mb
        // let mut last_row: Vec<i32> = vec![1; 1];
        // let row_index = row_index as usize + 1;
        // for i in 2..=row_index {
        //     let mut row = vec![1; i];
        //     if i > 2 {
        //         for j in 1..i - 1 {
        //             row[j] = last_row[j] + last_row[j - 1];
        //         }
        //     }
        //     last_row = row;
        // }
        // last_row

        // 方法2
        // 递归思想是一个很重要的思想
        // 是很多算法的基础，是程序思维的开端之一
        // 递归有两个要素，出口和处理
        // 出口也叫基线，通常是最简单的情况，
        //    本题就是当处于第0行的时候（从0开始数行）
        // 处理就是将递归作为一个整体或者处于某个问题的中间过程来看待，
        //    本题就是第N行，我们通过递归得到了N-1行，那么怎么计算第N行
        //    当然是第一个数据为1，然后从1开始迭代N-1行的数据，
        //    两两相加然后放入本行的结果，最后再添加1即可
        // AC 0ms 2mb
        if row_index == 0 { return vec![1]; }
        let prev_row = Self::get_row(row_index - 1);
        let mut row = vec![1];
        (1..prev_row.len()).for_each(|i| row.push(prev_row[i - 1] + prev_row[i]));
        row.push(1);
        row
    }
}