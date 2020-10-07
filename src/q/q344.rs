use crate::q::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // 方法1
        // 双指针，直接交换
        // Passed 20ms 5.3mb
        let (mut le, mut ri) = (0, s.len() as i32 - 1);
        while le < ri {
            s.swap(le as usize, ri as usize);
            le += 1;
            ri -= 1;
        }
    }
}

#[test]
fn test() {
    let mut v = vec![];
    Solution::reverse_string(&mut v);
    assert_eq!(v, vec![]);

    let mut v = vec!['h'];
    Solution::reverse_string(&mut v);
    assert_eq!(v, vec!['h']);

    let mut v = vec!['h', 'e'];
    Solution::reverse_string(&mut v);
    assert_eq!(v, vec!['e', 'h']);
}