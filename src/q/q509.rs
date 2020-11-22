use crate::q::Solution;

#[allow(unused)]
impl Solution {
    fn fib(n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        if n == 1 || n == 2 {
            return 1;
        }
        let mut sum = 1;
        let mut last = 1;
        for _ in 3..=n {
            let tmp = sum;
            sum += last;
            last = tmp;
        }
        sum
    }
}