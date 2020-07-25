use crate::Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        // 方法1
        // 由于数字可能较大，采用递归的方式会造成调用栈溢出，所以用循环来解决
        // 如果n = 0 | 1，那么返回n
        // n > 1的时候，令f0 = 0, f1 = 1, fn = 1，循环2到n，
        // 每次 fn = (f0 + f1) % 1000000007， f0 = f1, f1 = fn
        // 每次都取模可以很好的避免数字溢出，不过由于rust支持u128，所以
        // 在条件允许的情况下，我们也可以尝试u128来承载计算结果，最后结果再取模
        // Passed 0ms 2.1mb
        if n <= 1 { return n; }
        let (mut p0, mut p1, mut pn) = (0, 1, 0);
        (2..=n).for_each(|_| {
            pn = (p0 + p1) % 1000000007;
            p0 = p1;
            p1 = pn;
        });
        pn
    }
}

#[test]
fn test() {
    assert_eq!(Solution::fib(0), 0);
    assert_eq!(Solution::fib(1), 1);
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
    assert_eq!(Solution::fib(5), 5);
    assert_eq!(Solution::fib(6), 8);
}