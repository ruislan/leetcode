use crate::offer::Solution;

impl Solution {
    // 不能使用乘除法、for、while、if、else、switch、case等关键字及条件判断语句（A?B:C）
    // Rust应该是match之类的也不能用
    pub fn sum_nums(n: i32) -> i32 {
        // 方法1
        // 分析一下，不能用乘除，那就是可以用加减
        // 等差数列的求和公式是 n * (An + 1) / 2
        // 这里等差数列正好是1，带入公式得 n * (n + 1) / 2
        // 不能用乘除，除以2可以用向右移位操作得 n * (n + 1) >> 1
        // 这里还有一个乘号，观察得到n * (n + 1)实际上可以变换为 n.pow(2) + n
        // 那么n的平方应该不算乘法吧？
        // Passed 0ms 2.1mb
        // (n.pow(2) + n) >> 1

        // 方法2
        // 如果方法1不算正确的处理，那么来看看递归
        // 递归就是  if n > 0 { n + sum_nums(n - 1) } else { 0 }
        // 利用&&和||的断路原理，如果 a && b，a 为 false就不会判断b，否则就会判断b
        let mut n = n;
        n > 0 && {
            n += Self::sum_nums(n - 1);
            n > 0
        };
        n
    }
}

#[test]
fn test() {
    assert_eq!(Solution::sum_nums(1), 1);
    assert_eq!(Solution::sum_nums(3), 6);
    assert_eq!(Solution::sum_nums(4), 10);
    assert_eq!(Solution::sum_nums(100), 5050);
}