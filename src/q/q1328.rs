use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        // 方法1
        // 暴力法
        // 直接将每个字符串从a改到z，
        // 判断修改之后如果不是回文那么
        // 然后比较与当前字典最小的比较即可
        // 最后返回字典最小
        // 时间O(n * n * 26)
        // 只有1000个字符，暴力法完全可以处理


        // 方法2
        // 首先破坏回文只需要修改一个字符即可，
        // 然后如果要字典顺序，那么肯定尽量将靠近左边的非'a'字母修改成'a'即可
        // 这里只有两个例外，
        // 第一个都是'a'的情况，那么我们只需要将最后一个字母改成'b'就是最小字典顺序
        // 第二个就是回文是奇数的情况，那么如果恰好中间那个不是'a'，我们修改之后其实还是回文
        // 所以，我们要剔除掉中间那个数字，这里需要点小技巧
        // 我们可以利用迭代的时候i生成一个对面的j，就像j从右向左在遍历一样
        // 那么当i == j的时候，就是两个在中间相遇的时候，那个字符就可以忽略，
        // 如果是偶数的回文，i就无法与j相等，他们会在中间完美错过。
        if palindrome.len() < 2 { return String::new(); }
        let mut s: Vec<char> = palindrome.chars().collect();
        let n = s.len();
        for i in 0..n {
            if i != n - 1 - i { // if not the mid
                if s[i] != 'a' {
                    s[i] = 'a';
                    return s.into_iter().collect();
                }
            }
        }
        s[n - 1] = 'b'; // case "aaa"
        s.into_iter().collect()
    }
}