use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        // 要注意有连续在字符串中的情况，例如, au在lau中,lau在alau中
        // 方法1
        // 按照words的长度排序
        // 然后依次迭代words，当words[i]属于words[(i + 1)..]时，记录进这个结果
        // 排序O(nlogn)，迭代words是O(n*n)
        // let mut res = Vec::new();
        // for i in 0..words.len() {
        //     for j in 0..words.len() {
        //         if i != j && words[j].len() > words[i].len() && words[j].contains(&words[i]) {
        //             res.push(words[i].clone());
        //             break;
        //         }
        //     }
        // }
        // res

        // 也可以不对words排序，迭代words时，只有words[i]的长度小于words[j]的长度，才进行contains比较，然后记录结果(HashSet)
        // 迭代words并比较是O(n*n)
        // Passed 4ms 2.1mb
        let mut words = words;
        words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        let mut res = Vec::new();
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if words[j].contains(&words[i]) {
                    res.push(words[i].clone());
                    break;
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::string_matching(vec!["mass".to_string(), "as".to_string(), "hero".to_string(), "superhero".to_string()]), vec!["as".to_string(), "hero".to_string()]);
    assert_eq!(Solution::string_matching(vec!["leetcoder".to_string(), "leetcode".to_string(), "od".to_string(), "hamlet".to_string(), "am".to_string()]), vec!["od".to_string(), "am".to_string(), "leetcode".to_string()]);
    assert_eq!(Solution::string_matching(vec!["au".to_string(), "lau".to_string(), "alau".to_string()]), vec!["au".to_string(), "lau".to_string()]);
}