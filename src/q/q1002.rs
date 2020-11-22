use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        // 方法1
        // let mut map = std::collections::HashMap::new();
        // for ch in a[0].chars() {
        //     let mut count = map.entry(ch).or_insert(0);
        //     *count += 1;
        // }
        //
        // for i in 1..a.len() {
        //     let mut intra = Vec::new();
        //     for ch in a[i].chars() {
        //         if let Some(x) = map.get_mut(&ch) {
        //             *x -= 1;
        //             intra.push(ch);
        //             if *x == 0 { map.remove(&ch); }
        //         }
        //     }
        //     map.clear();
        //     for ch in intra {
        //         let mut count = map.entry(ch).or_insert(0);
        //         *count += 1;
        //     }
        // }
        //
        // let mut res = Vec::new();
        // for (k, v) in map {
        //     for i in 0..v {
        //         res.push(k.to_string());
        //     }
        // }
        // return res;

        // 方法2
        // let mut bags = Vec::new();
        //
        // for i in 0..a.len() {
        //     let mut bag = vec![0; 26];
        //     for ch in a[i].chars() {
        //         bag[ch as usize - 'a' as usize] += 1;
        //     }
        //     bags.push(bag);
        // }
        //
        // let mut res = Vec::new();
        // for i in 0..26 {
        //     let mut min = bags.len();
        //     for bag in bags.iter() {
        //         if min > bag[i] { min = bag[i]; }
        //     }
        //     for _ in 0..min {
        //         res.push(((i as u8 + 'a' as u8) as char).to_string());
        //     }
        // }
        // return res;

        // 方法3
        let mut bag = vec![0; 26];
        for ch in a[0].chars() {
            bag[ch as usize - 97] += 1;
        }

        for i in 1..a.len() {
            let mut chars = vec![0; 26];
            for ch in a[i].chars() {
                chars[ch as usize - 97] += 1;
            }
            for j in 0..26 {
                bag[j] = bag[j].min(chars[j]);
            }
        }

        let mut res = Vec::new();
        for i in 0..26 {
            for _ in 0..bag[i] {
                res.push(((i as u8 + 97) as char).to_string());
            }
        }

        res
    }
}

#[test]
fn test() {
    let a: Vec<String> = Vec::new();
    assert_eq!(Solution::common_chars(vec!["a".to_string()]), vec!["a".to_string()]);
    assert_eq!(Solution::common_chars(vec!["a".to_string(), "b".to_string()]), a);
    assert_eq!(Solution::common_chars(vec!["a".to_string(), "aaa".to_string(), "aa".to_string()]), vec!["a".to_string()]);
    assert_eq!(Solution::common_chars(vec!["bella".to_string(), "label".to_string(), "roller".to_string()]), vec!["e".to_string(), "l".to_string(), "l".to_string()]);
    assert_eq!(Solution::common_chars(vec!["cool".to_string(), "lock".to_string(), "cook".to_string()]), vec!["c".to_string(), "o".to_string()]);
    assert_eq!(Solution::common_chars(vec!["azzco".to_string(), "zco".to_string(), "azo".to_string()]), vec!["o".to_string(), "z".to_string()]
    );
}