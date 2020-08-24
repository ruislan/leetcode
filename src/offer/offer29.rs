use crate::offer::Solution;

// 同q54
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
        //  顺序打印xMax[y1:yMax]，
        //  逆序打印yMax[xMax - 1 : x0]
        //  逆序打印x0[yMax - 1: y0 + 1]
        //  然后令y0 += 1, x0 += 1, xMax -= 1, yMax -= 1
        //
        let mut res = Vec::new();
        res
    }
}