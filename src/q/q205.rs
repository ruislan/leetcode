use crate::q::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // 方法1
        // 如果s和t的长度不同，则直接返回false
        // 将s和t按照(char => indexes)的方式分别存储到hashmap中，为map_s，map_t
        // 再创建长度为s.len()的数组arr_s和arr_t，迭代map_s和map_t，按照indexes的第一个数字的位置存储到arr的位置上
        // 再判断arr_s和arr_t是否相等
        // Passed 0ms 4.3mb
        if s.len() != t.len() { return false; }

        let (mut map_s, mut map_t) = (std::collections::HashMap::new(), std::collections::HashMap::new());
        s.chars().enumerate().for_each(|(i, ch)| {
            let freq = map_s.entry(ch).or_insert(vec![]);
            freq.push(i);
        });
        t.chars().enumerate().for_each(|(i, ch)| {
            let freq = map_t.entry(ch).or_insert(vec![]);
            freq.push(i);
        });

        let (mut arr_s, mut arr_t) = (vec![vec![]; s.len()], vec![vec![]; t.len()]);
        map_s.into_iter().for_each(|(_, v)| {
            arr_s[v[0]] = v.to_owned();
        });
        map_t.into_iter().for_each(|(_, v)| {
            arr_t[v[0]] = v.to_owned();
        });

        arr_s == arr_t
    }
}

#[test]
fn test() {
    assert_eq!(Solution::is_isomorphic("".to_string(), "".to_string()), true);
    assert_eq!(Solution::is_isomorphic("a".to_string(), "b".to_string()), true);
    assert_eq!(Solution::is_isomorphic("egg".to_string(), "add".to_string()), true);
    assert_eq!(Solution::is_isomorphic("foo".to_string(), "bar".to_string()), false);
    assert_eq!(Solution::is_isomorphic("foo".to_string(), "foo".to_string()), true);
    assert_eq!(Solution::is_isomorphic("paper".to_string(), "title".to_string()), true);
}