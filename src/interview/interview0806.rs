use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        // 方法1
        // 汉诺塔是典型的递归问题
        // 递归就要分析基线条件，和递归过程
        // 基线条件肯定就是只有1个盘子的时候，直接将这个盘子从a 移动到 c即可
        // 那么递归过程就是当大于一个盘子的时候，我们先分析两个盘子和三个盘子的情况
        // 以方便我们归纳递归过程
        // 两个盘子的线路是： a -> b,               ; a -> c; b -> c
        // 三个盘子的线路是： a -> c, a -> b, c -> b; a -> c; b -> a, b -> c, a -> c
        // 可以看出来，我们第一步就是想办法将a除了最后1个盘子放到b去；
        // 第二步是将a的那个盘子放到c
        // 第三步是将b之前a移动过来的所有盘子放到c去；
        // 三个盘子的时候，a -> b，我们利用了c过渡，然后b -> c的时候利用了a过度
        // 所以我们假设出发的盘子是begin，目的地是end，过度的是temp
        // 那么递归的过程就是：
        //    将n - 1个盘子从begin移动到temp，以end过度
        //    将1个盘子从begin直接移动到end,不需要过度
        //    将之前移动过来的n - 1个盘子从temp移动到end，以begin过度
        fn mv(begin: &mut Vec<i32>, temp: &mut Vec<i32>, end: &mut Vec<i32>, n: usize) {
            if n == 1 {
                end.push(begin.pop().unwrap());
            } else {
                mv(begin, end, temp, n - 1);
                mv(begin, temp, end, 1);
                mv(temp, begin, end, n - 1);
            }
        }
        mv(a, b, c, a.len());
    }
}