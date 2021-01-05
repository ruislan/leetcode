use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        // 方法1
        // 设置起始位置start为0，然后从位置1开始迭代String的字符
        // 如果s[i] != s[i - 1]，说明连续字符中断，那么
        // 判断当前位置减去start位置是否大于等于3（也即是是否连续三个字符）
        // 如果是，则放入结果数组
        // 需要注意的是在迭代完成之后还要判断一下最后一个字符位置到start是否符合条件
        let mut answer = Vec::new();
        let mut n = s.len();
        let mut start = 0;
        let s = s.into_bytes();
        for i in 1..=n {
            if i == n || s[i - 1] != s[i] {
                if i - start >= 3 {
                    answer.push(vec![start as i32, i as i32 - 1]);
                }
                start = i;
            }
        }
        answer
    }
}

#[test]
fn test_q830() {
    assert_eq!(
        vec![vec![3, 5], vec![6, 9], vec![12, 14]],
        Solution::large_group_positions("abcdddeeeeaabbbcd".to_string())
    );
}
