use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        // 方法1
        // 计算一共有几周，然后多了几天
        // 我们就可以先通过每周多7来计算整的周数的统计
        // 然后我们再计算天数，天数就正好是周数开始的，
        // 例如第一周就是1，2，3，4，5，6，7
        // 第二周就是2,3,4,5,6,7,8……
        // 所以第n周的顺序就是(n - 1) + 1, (n - 1) + 2...
        // 当然应该还有更简单的公式，这里直接用程序去计算就行了
        let weeks = n / 7;
        let days = n % 7;
        let mut answer = 0;
        for week in 1..=weeks {
            answer += 28 + (week - 1) * 7;
        }
        for day in 1..=days {
            answer += weeks + day;
        }
        answer
    }
}