use crate::q::Solution;

#[allow(unused)]
impl Solution {
    /**
     * 在一个由小写字母构成的字符串S中，包含由一些连续的相同字符所构成的分组。
     *
     * 例如，在字符串 S = "abbxxxxzyy"中，就含有 "a", "bb", "xxxx", "z" 和 "yy" 这样的一些分组。
     * 我们称所有包含大于或等于三个连续字符的分组为较大分组。找到每一个较大分组的起始和终止位置。
     * 最终结果按照字典顺序输出。
     */
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut last_ch = None;
        let (mut p0, mut p1) = (0, 0);
        for ch in s.char_indices() {
            let pos = ch.0 as i32;
            let ch = ch.1;
            if None == last_ch {
                last_ch = Some(ch);
                p0 = pos;
            }
            if Some(ch) == last_ch {
                p1 = pos;
            } else {
                if p1 - p0 >= 2 {
                    res.push(vec![p0, p1]);
                }
                last_ch = Some(ch);
                p0 = pos;
                p1 = 0;
            }
        }
        if p1 - p0 >= 2 {
            res.push(vec![p0, p1]);
        }
        res
    }
}

#[test]
fn test_q830() {
    assert_eq!(
        vec![vec![3, 5], vec![6, 9], vec![12, 14]],
        Solution::large_group_positions("abcdddeeeeaabbbcd".to_string())
    );
}
