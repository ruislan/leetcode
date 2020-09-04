use crate::interview::Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // 方法1
        // 矩阵为空，不需要操作
        // 设置两个变量begin=0和end=n-1，分别表示横竖的起点和终点，有
        // a = col[begin]row[begin..end-1]
        // b = col[begin..end-1]row[end]
        // c = col[end]row[begin+1..end]
        // d = col[begin+1..end]row[begin]
        // 旋转意味着，
        // 当n = 1时（begin >= end），不需要旋转，因为n * n 的矩阵，只有1个数是不需要旋转的
        // 当n > 1时，
        // a b c d 顺时针变成 ~d a ~b c
        // 如果不能使用额外的空间来进行交换，那么可以采用
        // a与b交换，a与c交换，a与d交换即完成了所有的交换，注意原b与d交换的时候位置是要反着的
        // 每转换一次，begin += 1, end -= 1
        // 如果begin >= end，就不需要再操作了
    }
}