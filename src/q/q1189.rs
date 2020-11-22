use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // 方法二：记录word每个字符的数量，o,l两个字符除以2，然后找出5个字符中频次最小的那个
        // Passed 0ms 2.0mb
        let mut word = vec![0; 5];
        for ch in text.chars() {
            match ch {
                'b' => word[0] += 1,
                'a' => word[1] += 1,
                'l' => word[2] += 1,
                'o' => word[3] += 1,
                'n' => word[4] += 1,
                _ => (),
            }
        }
        word[2] /= 2;
        word[3] /= 2;
        *word.iter().min().unwrap()

        // 方法一： 记录字母频次，然后减去word，统计一共减去了多少个word
        // Passed 0ms 2.0mb
        // let mut bag = vec![0; 26];
        // for ch in text.chars() {
        //     bag[ch as usize - 97] += 1;
        // }
        // let mut sum = 0;
        // let (ch_b, ch_a, ch_l, ch_o, ch_n) = ('b' as usize - 97, 'a' as usize - 97, 'l' as usize - 97, 'o' as usize - 97, 'n' as usize - 97);
        // loop {
        //     bag[ch_b] -= 1;
        //     bag[ch_a] -= 1;
        //     bag[ch_l] -= 2;
        //     bag[ch_o] -= 2;
        //     bag[ch_n] -= 1;
        //     if bag[ch_b] < 0 || bag[ch_a] < 0 || bag[ch_l] < 0 || bag[ch_o] < 0 || bag[ch_n] < 0 { break; }
        //     sum += 1;
        // }
        // sum
    }
}

#[test]
pub fn test_q1189() {
    assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
    assert_eq!(2, Solution::max_number_of_balloons("loonbalxballpoon".to_string()));
    assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
    assert_eq!(0, Solution::max_number_of_balloons("nlaebolk".to_string()));
    assert_eq!(0, Solution::max_number_of_balloons("a".to_string()));
}