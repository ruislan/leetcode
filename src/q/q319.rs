use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        // 方法1
        // 通过从0个灯泡到25个灯泡模拟发现，
        // 0：0；1-3：1；4-8：2；9-15：3；16-24：4；25：5
        // 亮着的灯泡正好是向下看齐的某个整数的平方数
        // AC 0ms 1.9mb 35/35
        // let n = n as i64;
        // let mut ans = 0_i64;
        // while ans * ans <= n {
        //     ans += 1;
        // }
        // ans as i32 - 1

        // 方法2
        // 方法1换句话说，就是n的平方根向下取整就行
        // AC 0ms 1.9mb 35/35
        (n as f64).sqrt() as i32
    }
}
