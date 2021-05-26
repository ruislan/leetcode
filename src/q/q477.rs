use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        // 方法1
        // 两层循环暴力解决，数据量10^4，n^2就是10^8
        // 相信了一把Rust的效率，不会TLE，还真没有TLE，但是这明显不是这题的考点
        // AC 1064ms 2mb
        // let n = nums.len();
        // let mut answer = 0;
        // for i in 0..n {
        //     for j in i + 1..n {
        //         answer += (nums[i] ^ nums[j]).count_ones() as i32;
        //     }
        // }
        // answer

        // 方法2
        // 通常位运算都要看位，所以我们应该把所有的数字都拆开成bit
        // 范例： 4 14 2 答案是6
        //      0100
        //      1110
        //      0010
        // -----------
        //      0110   (6)
        //      2220  （每个位置的汉明距离正好是2+2+2+0=6）
        //  这里我们就可以看出来了，zero的个数和one的个数相乘就是单个位置的汉明距离
        //  最后结果就是所有的汉明距离的和
        //  时间复杂度O(32*n)
        //  AC 4ms 2.2mb
        let n = nums.len();
        let mut answer = 0;
        for i in 0..32 {
            let mut ones = 0;
            for j in 0..n {
                ones += (nums[j] >> i) & 1;
            }
            answer += ones * (n as i32 - ones);
        }
        answer
    }
}