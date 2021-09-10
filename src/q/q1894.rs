use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        // 方法1
        // 求出和sum（注意溢出，用i64)
        // k % rem得到余数rem
        // rem 依次对比chalk，rem - chalk[i] < 0的那个，就是结果
        let n = chalk.len();
        let mut sum = chalk.iter().map(|&x| x as i128).sum::<i128>();
        let mut rem = ((k as i128) % sum) as i32;
        chalk.iter().enumerate().find(|&(i, &x)| {
            rem -= x;
            rem < 0
        }).unwrap().0 as i32
    }
}

