struct Solution;

impl Solution {
    pub fn max_power(s: String) -> i32 {
        // 方法1
        // 设置一个初始的max = 0; count = 0;
        // 查找当前字符与上一个字符是否相等，相等就count += 1，如果不相等，就令count = 1，
        // 然后比较count和max，比max大就替换
        // 利用for i in 1..s.len():
        //       if &s[i - 1] == &s[i]:
        //          count += 1;
        //          max = max.max(count).unwrap();
        //       else ...


        // 方法2
        // 试试看能不能利用partition来分组，然后直接取最大长度的组？这个不确定
    }
}