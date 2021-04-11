use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // 方法1
        // 这道题值得回味哈
        // 遇到这样的题，我们需要找规律了
        // 先列出一些丑数，1，2，3，4，5，6，8，9，10，12
        // 这里我们知道丑数*2/3/5肯定还是个丑数
        // 所以，我们可以从第一个丑数开始，
        // 1 * 2 = 2, 1 * 3 = 3, 1 * 5 = 5
        // 这里如果我们直接放上去就没有4，而我们知道4 = 2 * 2
        // 也就是说，我们不能直接用1 * 2/3/5的方式直接列后面三个
        // 既然不能列出3个，那么1个可不可以
        // 我们观察第一个丑数的* 2/3/5就是2/3/5，那么第二个是应该是2
        // 而2 * 2/3/5 = 4/6/10，而我们第三个丑数又用的是3
        // 而3 * 2/3/5 = 6/9/15，这个地方我们取的是4
        // 这意味着，我们要记录一下2 * 2/3/5的那一个
        // 既然如此，那么2 * 3是不是要记录一下，2 * 5是不是要记录一下
        // 这样一来，我们需要记录某个数* 2/3/5的位置，分别为i_u2,i_u3,i_u5
        // 而我们要取的第n个丑数，就是min (ugly[i_u2] * 2, ugly[i_u3] * 3, ugly[i_u5] * 5)
        // 然后再根据我们选择的那个丑数，移动i_u2/i_u3/i_u5的位置
        // 这样一来 1 2 3 4（2*2） 5 6（3*2）8（4*2）9（3*3）10（5*2）12（6*2）
        // 直到算出第n个丑数ugly[n]
        // AC 0ms 2.1mb
        let n = n as usize;
        let mut ugly = vec![1; n];
        let (mut i_u2, mut i_u3, mut i_u5) = (0, 0, 0);
        for i in 1..n {
            let next_u2 = ugly[i_u2] * 2;
            let next_u3 = ugly[i_u3] * 3;
            let next_u5 = ugly[i_u5] * 5;
            let next = next_u2.min(next_u3).min(next_u5);
            if next == next_u2 { i_u2 += 1; }
            if next == next_u3 { i_u3 += 1; }
            if next == next_u5 { i_u5 += 1; }
            ugly[i] = next;
        }
        ugly[n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::nth_ugly_number(1), 1);
    assert_eq!(Solution::nth_ugly_number(4), 4);
    assert_eq!(Solution::nth_ugly_number(7), 8);
    assert_eq!(Solution::nth_ugly_number(10), 12);
    assert_eq!(Solution::nth_ugly_number(1690), 2123366400);
}