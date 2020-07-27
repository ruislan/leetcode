use crate::Solution;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        // 方法1
        // 创建一个字符串数组arr，长度与indices一样，迭代indices，有索引i，对应&s[i]放到arr[indices[i]]中
        // 返回arr组合成的字符串
        // 注意:如果只是ascii的字符的话，&s[i..=i]的取值完全没问题，但是如果有Unicode字符，就必须用chars()来解决了
        let mut arr = vec![' '; indices.len()];
        indices.iter().zip(s.chars()).for_each(|(&x, ch)| arr[x as usize] = ch);
        arr.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::restore_string("".to_string(), vec![]), "".to_string());
    assert_eq!(Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]), "leetcode".to_string());
    assert_eq!(Solution::restore_string("abc".to_string(), vec![0, 1, 2]), "abc".to_string());
    assert_eq!(Solution::restore_string("aiohn".to_string(), vec![3, 1, 4, 2, 0]), "nihao".to_string());
    assert_eq!(Solution::restore_string("aaiougrt".to_string(), vec![4, 0, 2, 6, 7, 3, 1, 5]), "arigatou".to_string());
    assert_eq!(Solution::restore_string("art".to_string(), vec![1, 0, 2]), "rat".to_string());
}