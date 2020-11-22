use crate::offer::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_continuous_sequence(target: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 设一个le，一个ri, le = 1, ri = le + 1
        // 循环，退出条件是le + ri > target
        // 我们逐步向右滑动ri,
        // a.当[le,ri].sum() = target时，将结果放到数组arr中
        // b.当[le,ri].sum() > target时，le += 1, ri = le + 1
        // 重复a,b，直到退出循环
        // Passed 0ms 2.1mb
        //
        // let (mut le, mut ri) = (1, 2);
        // let mut res = Vec::new();
        // loop {
        //     let mut sum = le + ri;
        //     if sum > target { break; }
        //
        //     while sum < target {
        //         ri += 1;
        //         sum += ri;
        //     }
        //
        //     if sum == target {
        //         res.push((le..=ri).collect());
        //     }
        //     le += 1;
        //     ri = le + 1;
        // }
        // res

        // 方法2
        // 其实我们可以看到方法1，每次都要求一个和挺费事的，实际上有些数字的和已经求了很多次了
        // 我们完全可以省略这些求和，我们通过观察可以看到，实际上每次要么是ri + 1，要么是le + 1
        // 不会同时ri + 1和le + 1，所以就呈现的感觉就像是le去追赶ri，追上就嘿嘿嘿？，不，追上就结束
        // 这样看起来就像一个橡皮筋，先是向右拉，拉过了target，左边就松一点，又不够target了，右边又多一点
        // 那你说我们每次是不是还是要求和吗？
        // 其实不用了，松一点，我们就减去le，加一点，我们加上ri + 1后的ri。
        // 当然，最好还是等差数列求和公式（如果你能够记得的话），Sn = (le + ri)*(ri -le + 1) / 2
        // Passed 0ms 2.1mb
        // let (mut le, mut ri, mut sum) = (1, 2, 3);
        // let mut res = Vec::new();
        // while le < ri {
        //     if sum < target {
        //         ri += 1;
        //         sum += ri;
        //     } else {
        //         if sum == target { res.push((le..=ri).collect()); }
        //         sum -= le;
        //         le += 1;
        //     }
        // }
        // res

        // 方法2+
        // 根据求和公式来计算
        // Passed 0ms 2.1mb
        let (mut le, mut ri) = (1, 2);
        let mut res = Vec::new();
        while le < ri {
            let sum = (le + ri) * (ri - le + 1) / 2;
            if sum < target {
                ri += 1;
            } else {
                if sum == target { res.push((le..=ri).collect()); }
                le += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let v: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::find_continuous_sequence(1), v);
    assert_eq!(Solution::find_continuous_sequence(2), v);
    assert_eq!(Solution::find_continuous_sequence(3), vec![vec![1, 2]]);
    assert_eq!(Solution::find_continuous_sequence(4), v);
    assert_eq!(Solution::find_continuous_sequence(5), vec![vec![2, 3]]);
    assert_eq!(Solution::find_continuous_sequence(6), vec![vec![1, 2, 3]]);
    assert_eq!(Solution::find_continuous_sequence(7), vec![vec![3, 4]]);
    assert_eq!(Solution::find_continuous_sequence(8), v);
    assert_eq!(Solution::find_continuous_sequence(9), vec![vec![2, 3, 4], vec![4, 5]]);
    assert_eq!(Solution::find_continuous_sequence(10), vec![vec![1, 2, 3, 4]]);
    assert_eq!(Solution::find_continuous_sequence(11), vec![vec![5, 6]]);
    assert_eq!(Solution::find_continuous_sequence(12), vec![vec![3, 4, 5]]);
}