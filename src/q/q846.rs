use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        // 方法1
        // 排序hand然后，将所有的数字的频率存入hashmap
        // 依次迭代hand，如果当前数字没有量了，则继续迭代
        // 如果还有量，取出来处理，查看hashmap中是否存在，如果存在，则继续
        // 直到达到group_size，并去掉hashmap中的量
        // 如果没有达到group_size，则返回失败
        // AC 16ms 2.3mb 73/73
        use std::collections::HashMap;
        let n = hand.len();
        let mut hand = hand;
        hand.sort_unstable();
        let mut freq = HashMap::new();
        for i in 0..n {
            *freq.entry(hand[i]).or_insert(0) += 1;
        }
        for i in 0..n {
            let x = hand[i];
            let count = freq.get_mut(&x).unwrap();
            if *count > 0 {
                *count -= 1;
                for j in 1..group_size {
                    let y = x + j;
                    if let Some(count) = freq.get_mut(&y) {
                        if *count > 0 {
                            *count -= 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}