use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // 方法1
        // hashmap
        // O(n), O(n)
        // AC 0ms 2.3mb
        // let mut map = std::collections::HashMap::new();
        // for x in nums {
        //     *map.entry(x).or_insert(0) += 1;
        // }
        // map.into_iter().filter(|&(k, v)| v == 1).last().unwrap().0

        // 方法2
        // 进阶要求，时间O(n)和空间O(1)
        // 空间O(1)，那么我们就不能用额外的空间，那么就只有位操作了
        // 从这里开始，我们考虑每一位，如果某个数字出现了3次，那么这一位的位之和 % 3 = 0
        // 所以，这里状态会有三个，分别是出现1次，2次和3次。我们可以记录成00，01和10
        // 我们可以用one和two两个数字来分别表示这三个状态，one是低位，two是高位
        // 而三个状态的变化顺序必然是00 -> 01 -> 10 -> 00
        // 由于我们将one, two拆开了，所以这个状态变化实际上是
        //         two   one
        // 初始      0     0
        // 第一次    0     1
        // 第二次    1     0
        // 第三次    0     0
        // 这个时候每次遇到一个数字，我们就轮转一次状态，首先看one：
        // if one == 0:
        //   if two == 0:
        //     one = 1
        //   if two == 1:
        //     one = 0
        // if one == 1:
        //   if two == 0:
        //     one = 0
        //   if two == 1:
        //     这个状态1 1是不会出现的
        // 所以我们可以知道 one = one ^ x & !two
        // 然后我们第二位实际上要在one的基础上去处理：
        // 按照上面的方式，我们得到了: two = two ^ x & !one
        // 最后就只剩下1个了
        // 这就是O(n) 和 O(1)
        // AC 0ms 2.1mb
        let mut one = 0;
        let mut two = 0;
        for x in nums {
            one = (one ^ x) & !two;
            two = (two ^ x) & !one;
        }
        one
    }
}