use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn multiply(a: i32, b: i32) -> i32 {
        // 方法1
        // 直接递归，随便选a或者b即可
        // if a == 0 { return 0; }
        // b + Solution::multiply(a - 1, b)

        // 方法2
        // 稍微优化一下方法1，选择a和b中小的那个作为递归参数，
        // 减少递归深度
        // AC 0ms 2mb
        fn cal(a: i32, b: i32) -> i32 {
            if a == 0 {
                return 0;
            }
            b + cal(a - 1, b)
        }
        cal(a.min(b), a.max(b))
    }
}
