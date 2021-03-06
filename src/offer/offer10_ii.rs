use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_ways(n: i32) -> i32 {
        // 方法1
        // 假设有1步，那么就只有1种方式，
        // 假设有2步，那么就有[1,1]和[2]两种方式
        // 假设有3步，那么就有[1,1,1]、[1,2]、[2,1]三种方式
        // 假设有4步，那么就有[1,1,1,1]、[1,1,2]、[1,2,1]、[2,1,1]、[2,2]五种方式
        // 假设有5步，那么就有[1,1,1,1,1]、[2,1,1,1]、[1,2,1,1]、[1,1,2,1]、[1,1,1,2]、[2,2,1]、[2,1,2]、[2,2,1]八种方式
        // 这里我们基本上看出来了，每增加一步，实际上就是前两步所有方式的和
        // 即f(n) = f(n - 1) + f(n - 2)，这就是斐波那契数列啊
        // 令f0 = 0, f1 = 1，n = 0返回f0,n = 1返回f1，当n > 1，迭代2..=n，
        //     每次fn = (f0 + f1) % 1e9+7, f0 = f1, f1 = fn
        // 最后返回fn即可
        (1..=n).fold((0, 1), |(p0, p1), _| (p1, (p0 + p1) % 1000000007)).1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_ways(1), 1);
    assert_eq!(Solution::num_ways(2), 2);
    assert_eq!(Solution::num_ways(3), 3);
    assert_eq!(Solution::num_ways(4), 5);
    assert_eq!(Solution::num_ways(5), 8);
    assert_eq!(Solution::num_ways(6), 13);
    assert_eq!(Solution::num_ways(7), 21);
}