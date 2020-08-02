use crate::offer::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        // 方法1
        // 创建一个26长度的数组arr，迭代s，将char作为索引,(count,index)作为值存储进去
        // 迭代arr，找到index最小的且count = 1的
        // Passed 0ms 2.1mb
        let mut arr = vec![(0, s.len()); 26];
        s.char_indices().for_each(|(i, ch)| {
            let j = ch as usize - 97;
            arr[j].0 += 1;
            if i < arr[j].1 { arr[j].1 = i; }
        });
        let mut min_index = s.len();
        let mut min_char = ' ';
        arr.into_iter().enumerate().for_each(|(i, (count, j))| {
            if count == 1 && j < min_index {
                min_char = (i as u8 + 97) as char;
                min_index = j;
            }
        });
        min_char
    }
}

#[test]
fn test() {
    assert_eq!(Solution::first_uniq_char("".to_string()), ' ');
    assert_eq!(Solution::first_uniq_char("abaccdeff".to_string()), 'b');
    assert_eq!(Solution::first_uniq_char("a".to_string()), 'a');
    assert_eq!(Solution::first_uniq_char("ab".to_string()), 'a');
    assert_eq!(Solution::first_uniq_char("abc".to_string()), 'a');
    assert_eq!(Solution::first_uniq_char("aabbc".to_string()), 'c');
    assert_eq!(Solution::first_uniq_char("aabc".to_string()), 'b');
    assert_eq!(Solution::first_uniq_char("z".to_string()), 'z');
}