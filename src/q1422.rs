struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        // 方法1
        // 暴力解决方法，分割点从1..s.len() - 2，得到所有的值，取最大值

        // 方法2
        // 向右求出0的前缀和zeros，向左求出1的前缀和ones
        // zeros和ones的相同位置相加得到数组arr，skip头尾，返回arr[i]中的最大值即可
    }
}