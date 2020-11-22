use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        // 方法1
        // 排序，然后去掉头尾，求和再求平均数
        // Passed 0ms 2.1mb
        // let mut salary = salary;
        // salary.sort();
        // (1..salary.len() - 1).fold(0_f64, |acc, i| acc + salary[i] as f64) / (salary.len() - 2) as f64

        // 方法2
        // 先求整体和，再减去最大值和最小值，再求平均数
        (salary.iter().sum::<i32>() - *salary.iter().min().unwrap_or(&0) - *salary.iter().max().unwrap_or(&0)) as f64 / (salary.len() - 2) as f64
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::average(vec![]), 0);
    assert_eq!(Solution::average(vec![4000, 3000, 1000, 2000]), 2500_f64);
    assert_eq!(Solution::average(vec![1000, 2000, 3000]), 2000_f64);
    assert_eq!(Solution::average(vec![6000, 5000, 4000, 3000, 2000, 1000]), 3500_f64);
    assert_eq!(Solution::average(vec![8000, 9000, 2000, 3000, 6000, 1000]), 4750_f64);
}