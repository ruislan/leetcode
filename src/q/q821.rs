use crate::q::Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = Vec::new();
        let mut last_index_of_ch = -1i32;
        let mut cur_i = 0i32;
        for ch in s.chars() {
            if ch == c {
                if last_index_of_ch == -1i32 {
                    for i in (0..=cur_i).rev() {
                        res.push(i);
                    }
                } else {
                    let gap = cur_i - last_index_of_ch;
                    if gap > 1 {
                        let mid = (gap - 1) / 2;
                        for i in 1..=mid {
                            res.push(i);
                        }
                        for i in (1..(gap - mid)).rev() {
                            res.push(i);
                        }
                    }
                    res.push(0);
                }
                last_index_of_ch = cur_i;
            }
            cur_i += 1;
        }

        // tail
        if last_index_of_ch < cur_i - 1 {
            for i in 1..(cur_i - last_index_of_ch) {
                res.push(i);
            }
        }

        res
    }
}

#[test]
fn test_q821() {
    assert_eq!(
        vec![3i32, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
        Solution::shortest_to_char("loveleetcode".to_string(), 'e')
    );
    assert_eq!(vec![0i32, 0, 0], Solution::shortest_to_char("eee".to_string(), 'e'));
    assert_eq!(
        vec![0i32, 0, 0, 0, 0, 0, 0],
        Solution::shortest_to_char("eeeeeee".to_string(), 'e')
    );
    assert_eq!(vec![1i32, 0], Solution::shortest_to_char("ke".to_string(), 'e'));
    assert_eq!(vec![0i32, 1], Solution::shortest_to_char("ek".to_string(), 'e'));
    assert_eq!(vec![0i32], Solution::shortest_to_char("e".to_string(), 'e'));
    assert_eq!(
        vec![1i32, 0, 0, 0, 1],
        Solution::shortest_to_char("leeel".to_string(), 'e')
    );
}