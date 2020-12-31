use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 教科书般的贪心算法
        // 核心思想就是不重叠区间留下，重叠区间留下结束最早的
        // 先排序intervals，按照开始索引最小排，如果开始索引相等则结束最早的排前面
        // 然后第一个intervals[0]是必须要有的，
        // 后面的区间的开始时间最接近上一个区间结束时间的就排入
        // 如果这个区间的结束时间小于上一个结束区间，那么就采用这个区间，淘汰掉上一个区间
        // Passed 0ms 2.4mb
        // let n = intervals.len();
        // if n < 2 { return 0; }
        // let mut intervals = intervals;
        // intervals.sort_unstable_by(|a, b| {
        //     let cmp = a[0].cmp(&b[0]);
        //     if cmp == std::cmp::Ordering::Equal {
        //         a[1].cmp(&b[1])
        //     } else {
        //         cmp
        //     }
        // });
        //
        // let mut answer: Vec<Vec<i32>> = Vec::new();
        // for x in intervals {
        //     if let Some(prev) = answer.pop() {
        //         if x[0] >= prev[1] {
        //             answer.push(prev);
        //             answer.push(x);
        //         } else if x[1] < prev[1] {
        //             answer.push(x);
        //         } else {
        //             answer.push(prev);
        //         }
        //     } else {
        //         answer.push(x);
        //     }
        // }
        //
        // (n - answer.len()) as i32

        // 方法2
        // 按结束的早晚排序，我们每次都取结束最早的
        // 然后有重叠的话，就丢弃重叠的
        // 注意的是要有一个变量保存上一次的最右边在哪个位置
        // Passed 0ms 2.5mb
        let n = intervals.len();
        if n < 2 { return 0; }
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

        let mut answer = 1;
        let mut prev = intervals[0][1];
        for i in 1..n {
            if intervals[i][0] >= prev {
                answer += 1;
                prev = intervals[i][1];
            }
        }

        n as i32 - answer
    }
}