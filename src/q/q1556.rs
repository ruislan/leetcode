use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn thousand_separator(n: i32) -> String {
        // 方法1
        // 直接将数字转换为字符串，创建一个空字符串res，创建一个处理统计count
        // 将字符串倒序取出放入res，每隔3个(count%3==0)且不等于原字符串长度就加入一个点
        // 最后res倒叙并形成字符串
        let mut origin = n.to_string();
        let (len, mut count, mut res) = (origin.len(), 0, String::new());
        while let Some(ch) = origin.pop() {
            count += 1;
            res.push(ch);
            if count % 3 == 0 && count != len { res.push('.'); }
        }
        res.chars().rev().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::thousand_separator(0), "0");
    assert_eq!(Solution::thousand_separator(11), "11");
    assert_eq!(Solution::thousand_separator(111), "111");
    assert_eq!(Solution::thousand_separator(1234), "1.234");
    assert_eq!(Solution::thousand_separator(60010), "60.010");
    assert_eq!(Solution::thousand_separator(111111), "111.111");
    assert_eq!(Solution::thousand_separator(1111111), "1.111.111");
    assert_eq!(Solution::thousand_separator(i32::max_value()), "2.147.483.647");
}