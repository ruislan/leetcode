use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // 方法1
        // 将字母按照频率统计freq
        // 实际上任务的执行时间就是任务最多的那个任务，记为max
        // 由于最多的那个任务之间有n个空闲，所以这些空闲可以填充其他的不同的任务
        // 那些不同的任务我们可以忽略不计
        // 然后就只剩下最后一次执行max任务了，因为最后一次执行完之后就结束了，所以
        // 并没有执行后面的n的时间，但是也有可能这些时间在执行和max任务同样次数的任务
        // 所以我们也要统计一下这个任务的数量有多少个，记为max_times_count
        // 那么最后执行时间实际上就是 (max_times - 1) * (n + 1) + max_times_count
        // 现在有一个特殊的情况就是最后执行的时间居然比tasks.len()的时间还短
        // 例如 aaabbb n=0，max_times=3 max_times_count = 2,
        // 计算得到，(3 - 1) * (0 + 1) + 2 = 4，比len=6少2，
        // 这种情况下，我们只能选择tasks.len()
        // Passed 24ms 2.6mb
        let mut len = tasks.len();
        let mut freq = vec![0; 26];
        let base = 'A' as usize;
        tasks.into_iter().for_each(|x| freq[x as usize - base] += 1);
        let mut max_times = *freq.iter().max().unwrap();
        let mut max_times_count = freq.iter().filter(|&&x| x == max_times).count() as i32;
        (len as i32).max((max_times - 1) * (n + 1) + max_times_count)
    }
}