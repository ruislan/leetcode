use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 我们看到这个题的数据量很小，只有1..12，这说明这题多半是个比较耗时的
        // 做多了题，我们就知道，这个数据量，多半就是暴力，也就是回溯，可能还会有减枝。
        // 这里我们来想想一下怎么来做回溯，我们把每个工人看成一个箱子，而工作时间就是一个有重量的东西
        // 我们逐个箱子里面放东西，然后一旦我们成功放置了一次，那么就记录一次答案，这里最差就是一个人做了所有工作
        // 直到找到所有的方案，这里肯定会TLE，那么我们需要剪枝和记忆化
        // 第一个减枝就是如果我们已经找到一个答案，然后发现在搜索的过程中，比这个答案还大，那就没必要搜索了
        // 第二个记忆化，因为对于每个worker来说都是一样的，所以我们把任务分给任何一个人都是同等的路径，
        //            只是变成worker的id变成了0,1,2...而已，所以我们完全可以记忆这个子路径就可以了，
        //            出现过这个子路径，我们直接就可以用了，没出现过的，才继续。
        // AC 84ms 1.9mb
        // fn backtrace(workers: &mut Vec<i32>, max: i32, i: usize, jobs: &Vec<i32>, k: usize, answer: &mut i32) {
        //     if max > *answer { return; }
        //
        //     if i == jobs.len() {
        //         *answer = max.min(*answer);
        //         return;
        //     }
        //     let mut set = std::collections::HashSet::new();
        //     for j in 0..k {
        //         if !set.contains(&workers[j]) {
        //             workers[j] += jobs[i];
        //             backtrace(workers, max.max(workers[j]), i + 1, jobs, k, answer);
        //             workers[j] -= jobs[i];
        //             set.insert(workers[j]);
        //         }
        //     }
        // }
        // let k = k as usize;
        // let mut answer = i32::MAX;
        // backtrace(&mut vec![0; k], 0, 0, &jobs, k, &mut answer);
        // answer

        // 方法2
        // 我们换个角度来想问题，你给员工分配工作，那肯定是越平均越好，这样避免一个人做很多，
        // 想到这里，肯定第一想的是那就做个平均数，但是平均数不一定会在jobs里面sum出来，
        // 所以我们只能想个办法去猜出来，最简单的就是平均每个人1，2，3，4这样去猜，但是效率很低，
        // 那么，既然是顺序猜，那完全可以用二分猜嘛，这样效率就高太多了。
        // 二分的两个边界就是1和jobs.sum()，1表示最少得1个工作吧，而最大就是一个worker做了所有的工作
        // 我们每次都二分猜出这个是不是最优的分配方案，然后二分逼近这个答案
        // 大框架有了，那么，那么现在问题就在怎么判断这个方案可不可行上面了？
        // 这就只能靠回溯暴力搜索了，我们依次给workers分配工作
        // 如果工作分配超过了这个二分设定的方案，那么说明这个分配不行，
        // 如果这个分配当中只要有一次可行，那么就表示可行，那么就可以了
        // 但是这个暴力依然需要剪枝和记忆化，记忆化可以参考方法1
        // 剪枝就看当前这个worker的工作时间+给他的时间是不是比方案的值大，如果大，那就剪掉
        // AC 28ms 1.9mb
        // fn backtrace(workers: &mut Vec<i32>, max: i32, i: usize, jobs: &Vec<i32>, k: usize) -> bool {
        //     if i == jobs.len() { return true; }
        //
        //     let mut set = std::collections::HashSet::new();
        //     for j in 0..k {
        //         if workers[j] + jobs[i] > max { continue; }
        //         if !set.contains(&workers[j]) {
        //             workers[j] += jobs[i];
        //             if backtrace(workers, max, i + 1, jobs, k) { return true; }
        //             workers[j] -= jobs[i];
        //             set.insert(workers[j]);
        //         }
        //     }
        //     false
        // }
        //
        // let k = k as usize;
        // let mut lo = 1;
        // let mut hi = jobs.iter().sum::<i32>();
        // while lo < hi {
        //     let mid = lo + (hi - lo) / 2;
        //     if backtrace(&mut vec![0; k], mid, 0, &jobs, k) {
        //         hi = mid;
        //     } else {
        //         lo = mid + 1;
        //     }
        // }
        // lo

        // 方法2的另外一种优化
        // 这里我们用hashset来记录时间累计其实挺费时间的，我们可以直接给当前这个时间定一个标志
        // 如果这个时间积累出现过了，那么就不用给他了，否则就给他
        // AC 4ms 1.9mb
        // fn backtrace(workers: &mut Vec<i32>, max: i32, i: usize, jobs: &Vec<i32>, k: usize) -> bool {
        //     if i == jobs.len() { return true; }
        //     let mut flag = 0;
        //     for j in 0..k {
        //         if workers[j] + jobs[i] > max { continue; }
        //
        //         if workers[j] == 0 {
        //             if flag == 1 { continue; } else { flag = 1; }
        //         }
        //
        //         workers[j] += jobs[i];
        //         if backtrace(workers, max, i + 1, jobs, k) { return true; }
        //         workers[j] -= jobs[i];
        //     }
        //     false
        // }
        //
        // let k = k as usize;
        // let mut lo = 1;
        // let mut hi = jobs.iter().sum::<i32>();
        // while lo < hi {
        //     let mid = lo + (hi - lo) / 2;
        //     if backtrace(&mut vec![0; k], mid, 0, &jobs, k) {
        //         hi = mid;
        //     } else {
        //         lo = mid + 1;
        //     }
        // }
        // lo

        // 方法2的再一种优化
        // 我们其实可以把jobs排序，这样最大的工作先分配，这样可以保证最快的溢出检测
        // AC 0ms 2mb
        fn backtrace(workers: &mut Vec<i32>, max: i32, i: usize, jobs: &Vec<i32>, k: usize) -> bool {
            if i == jobs.len() { return true; }
            let mut flag = 0;
            for j in 0..k {
                if workers[j] + jobs[i] > max { continue; }

                if workers[j] == 0 {
                    if flag == 1 { continue; } else { flag = 1; }
                }

                workers[j] += jobs[i];
                if backtrace(workers, max, i + 1, jobs, k) { return true; }
                workers[j] -= jobs[i];
            }
            false
        }

        let mut jobs = jobs;
        jobs.sort_unstable_by(|a, b| b.cmp(a));
        let k = k as usize;
        let mut lo = 1;
        let mut hi = jobs.iter().sum::<i32>();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if backtrace(&mut vec![0; k], mid, 0, &jobs, k) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
}