use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        // 方法1
        // 将Radiant和Dire分成两队，记录下每队的出场序号
        // 这里有一个关键的思路就是，最好干掉序号紧挨着你的那个对手，这样能够保证你的权利最大化
        // 因为如果你干掉前面的，那已经行使过权利了，没啥意义
        // 如果你干掉比较后面的，他离形行使权利还很早，也没啥意义
        // 基于此思想模拟进程，每轮：
        //    两队分别派出选手比较序号，谁的序号小，谁就到自己那队的待留下的区域等待
        //    如果某个队伍空了，但是另外一个队伍还有，那么空的那个队伍的待留下区域里面的人就依次离开
        // 循环每轮直到某个队伍完全没有了选手
        // 还有选手的那个队伍就胜利了
        // 这里使用队列的原因，就是因为我们在待留下区域里面的人会先进先出
        // Passed 0ms 2.3mb
        // let mut senate_r = std::collections::VecDeque::new();
        // let mut senate_d = std::collections::VecDeque::new();
        // for (i, x) in senate.into_bytes().into_iter().enumerate() {
        //     if x == 'R' as u8 {
        //         senate_r.push_back(i);
        //     } else {
        //         senate_d.push_back(i);
        //     }
        // }
        //
        // while !senate_r.is_empty() && !senate_d.is_empty() {
        //     let mut remain_r = std::collections::VecDeque::new();
        //     let mut remain_d = std::collections::VecDeque::new();
        //     let mut i = 0;
        //     let mut j = 0;
        //     while i < senate_r.len() || j < senate_d.len() {
        //         if i >= senate_r.len() {
        //             remain_r.pop_front();
        //             remain_d.push_back(senate_d[j]);
        //         } else if j >= senate_d.len() {
        //             remain_d.pop_front();
        //             remain_r.push_back(senate_r[i]);
        //         } else if senate_r[i] > senate_d[j] {
        //             remain_d.push_back(senate_d[j]);
        //         } else {
        //             remain_r.push_back(senate_r[i]);
        //         }
        //         i += 1;
        //         j += 1;
        //     }
        //     senate_r = remain_r;
        //     senate_d = remain_d;
        // }
        //
        // String::from(if senate_d.is_empty() { "Radiant" } else { "Dire" })

        // 方法2
        // 优化方法1
        // 其实我们不用将留下的那个选手进入待留下区域，可以直接将他加在他那队伍的后面,他的值则增长一个队伍的长度
        // 这样可以保证一直比下去，直到一方空了为止
        // Passed 0ms 2.3mb
        let mut n = senate.len();
        let mut senate_r = std::collections::VecDeque::new();
        let mut senate_d = std::collections::VecDeque::new();
        for (i, x) in senate.into_bytes().into_iter().enumerate() {
            if x == 'R' as u8 {
                senate_r.push_back(i);
            } else {
                senate_d.push_back(i);
            }
        }

        while !senate_r.is_empty() && !senate_d.is_empty() {
            let r = senate_r.pop_front().unwrap();
            let d = senate_d.pop_front().unwrap();
            if r < d {
                senate_r.push_back(r + n);
            } else {
                senate_d.push_back(d + n);
            }
        }

        String::from(if senate_d.is_empty() { "Radiant" } else { "Dire" })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire".to_string());
    assert_eq!(Solution::predict_party_victory("RRDDD".to_string()), "Radiant".to_string());
    assert_eq!(Solution::predict_party_victory("DDRRR".to_string()), "Dire".to_string());
}