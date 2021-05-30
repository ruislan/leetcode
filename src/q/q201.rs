use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        // 方法1
        // 如果是for循环的话会TLE，利用fold函数来处理不会TLE
        // AC 1104ms 2.1mb
        // if left == right { return left; }
        // (left + 1..=right).fold(left, |acc, x| acc & x)

        // 方法2
        // and的特点就是有一个0，就不行了，
        // 所以我们需要观察left..=right之间有多少位置上是有0的
        // 如果一个连续数字的话，其实可以看出来left和right的二进制公共前缀是相同的，
        // 而右边部分一定都为0，因为连续上来的，一定会有一个数字的在某个位置上的二进制是0
        // 例如范例[5,6,7] 101 110 111 101和111的相等前缀是1，所以我们得到100
        // AC 8ms 2.2mb
        // let mut answer = 0;
        // for i in (0..31).rev() {
        //     let l = (left >> i) & 1;
        //     let r = (right >> i) & 1;
        //     if l != r { break; }
        //     answer |= l << i;
        // }
        // answer

        // 方法3
        // 还是最长公共前缀的方式，我们换一种方式来寻找
        // x & x - 1这个方法可以消除掉x最右边的1
        // 例如 [5,6] 101 & 110 = 100，这样6[110]的最右边的那一个1就消除了
        // 再如 [4,5] 100 & 101 = 100, 这样5[101]的最右边的1就消除了
        // 再如 [7,8] 0111 & 1000 = 0000，这样8[1000]只有一个1，就都被消除了
        // 所以当right <= left时，我们就找到公共前缀了，就是不停操作后的right
        // AC 4ms 1.9mb
        let mut left = left;
        let mut right = right;
        while left < right {
            right &= right - 1
        }
        right
    }
}