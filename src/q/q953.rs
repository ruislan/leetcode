use crate::q::Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut dic = std::collections::HashMap::new();
        let mut order_iter = order.char_indices();
        dic.insert(0, 0);
        while let Some((i, ch)) = order_iter.next() {
            dic.insert(ch as u8, i + 1);
        }
        for i in 1..words.len() {
            let len = std::cmp::max(words[i].len(), words[i - 1].len());
            let mut last = words[i - 1].bytes().collect::<Vec<u8>>();
            let mut cur = words[i].bytes().collect::<Vec<u8>>();
            for _ in last.len()..len {
                last.push(0);
            }
            for _ in cur.len()..len {
                cur.push(0);
            }

            for j in 0..len {
                let last_ch = dic.get(&last[j]).unwrap();
                let cur_ch = dic.get(&cur[j]).unwrap();
                if last_ch > cur_ch {
                    return false;
                }
                if cur_ch > last_ch {
                    break;
                }
            }
        }

        true
    }
}

#[test]
fn test_q953() {
    assert_eq!(
        false,
        Solution::is_alien_sorted(
            vec!["aa".to_string(), "a".to_string()],
            "abqwertyuioplkjhgfdszxcvnm".to_string(),
        )
    );
    assert_eq!(
        true,
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string(),
        )
    );
}