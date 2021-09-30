use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        // 方法1
        // 关键点在有没有覆盖区域，有覆盖区域，就需要减去覆盖即可
        // 判断有没有覆盖，我们可以从两个方向
        // x轴：如果a的最低x都比b的最高x大或等于，那么说明y轴怎么变都不会相交
        // 同理y轴也是一样
        // AC 4ms 2.1mb 3080/3080
        let area = (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1);
        if ax2 <= bx1 || bx2 <= ax1 || ay1 >= by2 || by1 >= ay2 {
            area
        } else {
            area - (ax2.min(bx2) - ax1.max(bx1)) * (ay2.min(by2) - ay1.max(by1))
        }
    }
}