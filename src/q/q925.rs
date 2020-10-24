use crate::q::Solution;

impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        // 方法1
        // 将两个字符串以ch:feq的形式存储到两个数组a,b中
        // 然后比较两个数组，如果a与b的字符串不同或者a的字符频率大于b的，则返回false
        // 最后返回 true
        // Passed 0ms 2.1mb
        // let mut n = Vec::new();
        // let mut ch = None;
        // let mut count = 0;
        // for chn in name.chars() {
        //     if None == ch {
        //         ch = Some(chn);
        //         count = 1;
        //     } else if Some(chn) == ch {
        //         count += 1;
        //     } else {
        //         n.push((ch.unwrap(), count));
        //         ch = Some(chn);
        //         count = 1;
        //     }
        // }
        // if count > 0 { n.push((ch.unwrap(), count)); }
        //
        // let mut t = Vec::new();
        // ch = None;
        // count = 0;
        // for cht in typed.chars() {
        //     if None == ch {
        //         ch = Some(cht);
        //         count = 1;
        //     } else if Some(cht) == ch {
        //         count += 1;
        //     } else {
        //         t.push((ch.unwrap(), count));
        //         ch = Some(cht);
        //         count = 1;
        //     }
        // }
        // if count > 0 { t.push((ch.unwrap(), count)); }
        //
        // if t.len() != n.len() { return false; }
        // for i in 0..t.len() {
        //     if t[i].0 != n[i].0 { return false; }
        //     if t[i].1 < n[i].1 { return false; }
        // }
        // return true;

        // 方法2
        // 今天轮到这个每日一题了，
        // 用kotlin做完才发现我半年前写的这代码太丑了，
        // 我决定重新搞一下,好看多了，下次再轮到这道题就用双指针解法吧
        // Passed 0ms 2.2mb
        let (mut name, typed) = (name.into_bytes(), typed.into_bytes());
        let (mut a, mut b) = (vec![(name[0], 1)], vec![(typed[0], 1)]);
        (1..name.len()).for_each(|i| { if name[i] != name[i - 1] { a.push((name[i], 1)); } else { a.last_mut().unwrap().1 += 1; } });
        (1..typed.len()).for_each(|i| { if typed[i] != typed[i - 1] { b.push((typed[i], 1)); } else { b.last_mut().unwrap().1 += 1; } });
        if a.len() != b.len() { return false; }
        a.into_iter().zip(b).all(|(a, b)| { a.0 == b.0 && a.1 <= b.1 })
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()), false);
}