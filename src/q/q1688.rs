use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        // 方法1
        // 递归处理，
        // 每轮比赛判断奇偶数
        // 如果是偶数 n / 2场比赛，剩下 n / 2队伍
        // 如果是奇数 (n - 1) / 2场比赛，剩下 (n - 1) / 2 + 1队伍
        // 然后把剩下的队伍继续递归
        // 递归基线，就是只有1个队伍的时候，就是出口
        // Passed 0ms 2mb
        // if n == 1 { return 0; }
        // if n & 1 == 1 {
        //     (n - 1) / 2 + Self::number_of_matches((n - 1) / 2 + 1)
        // } else {
        //     n / 2 + Self::number_of_matches(n / 2)
        // }

        // 方法2
        // 更好的递归
        // 不需要管奇偶，场次都相同，利用 (n + 1 )/ 2消灭奇偶影响
        // Passed 0ms 1.9mb
        // if n == 1 { return 0; }
        // n / 2 + Self::number_of_matches((n + 1) / 2)

        // 方法3
        // 数学方法
        // 既然每场比赛都会淘汰1个队伍，而最后只会剩下1个冠军
        // 那么就意味着要进行n - 1场比赛
        // Passed 0ms 1.9mb
        n - 1
    }
}