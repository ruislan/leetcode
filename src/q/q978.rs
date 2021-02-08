use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        // 方法1
        // 动态规划的特点就是局部最优解解会影响全局最优解
        // 这题的条件都是符合的
        // 为了便于理解，我们可以将所有的比较结果存储到op中
        // op[0]是“=”号，前面比后面大为“>”，前面比后面小为“<”，相等为“=”
        // 这样，我们可以直接用符号来确定最大“湍流”
        // 如果op[i] != 0，说明符号有效，
        //    前后符号不同，局部最优解+1，
        //    前后符号相同，局部最优解重新记录为1，
        // 如果op[i] == 0，说明符号无效，局部最优解重新记录为0
        // 每次检查符号之后，检查局部最优解是否是全局最优解
        // 最后返回全局最优解
        // 如果很娴熟了可以不要op来辅助，直接解决。
        // AC 8ms 2.3mb
        // let n = arr.len();
        // let mut op = vec![0; n];
        // for i in 1..n {
        //     let cmp = arr[i] - arr[i - 1];
        //     op[i] = if cmp > 0 {
        //         1
        //     } else if cmp < 0 {
        //         -1
        //     } else {
        //         0
        //     };
        // }
        // let mut answer = 0;
        // let mut max = 0;
        // for i in 1..n {
        //     if op[i] != 0 {
        //         if op[i] != op[i - 1] {
        //             max += 1;
        //         } else {
        //             max = 1;
        //         }
        //     } else {
        //         max = 0;
        //     }
        //     answer = answer.max(max);
        // }
        // answer + 1

        // 方法2
        // 用滑动窗口解决
        // 设置左右指针lo和hi，以及一个前面的比较结果，初始为相等
        // 滑动窗口，比较arr[hi - 1]和arr[hi]
        // 如果比较结果是相等，那么左右指针相等，窗口收缩到0
        // 如果比较结果于上一个比较结果相等，则左指针等于hi - 1，窗口收缩到1
        // 否则，右指针等于hi + 1，窗口扩展
        // 然后每次看是否是最大窗口
        // AC 4ms 2.4mb
        let n = arr.len();
        if n < 2 { return 1; }
        let mut lo = 0;
        let mut answer = 0;
        let mut prev_op = 0;
        for hi in 1..n {
            let cmp = arr[hi] - arr[hi - 1];
            let op = if cmp == 0 { 0 } else if cmp > 0 { 1 } else { -1 };
            if op != 0 && prev_op == op {
                lo = hi - 1;
            } else if op == 0 {
                lo = hi;
            }
            prev_op = op;
            answer = answer.max((hi - lo) as i32 + 1);
        }
        answer
    }
}