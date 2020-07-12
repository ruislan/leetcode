use crate::Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        // 方法1
        // 构建一个hashset，和初始点p(0,0)，并将初始点放入set
        // 迭代path，N则p.0 += 1，S则p.0 -=1，E则p.1 += 1，W则p.1 -=1
        // 如果这个计算后的点p在set中，则返回true
        // 循环结束，返回false
        let mut track = std::collections::HashSet::new();
        let mut pos = (0, 0);
        track.insert(pos);
        for direction in path.chars() {
            match direction {
                'N' => pos.0 += 1,
                'S' => pos.0 -= 1,
                'E' => pos.1 += 1,
                'W' => pos.1 -= 1,
                _ => ()
            };
            if !track.insert(pos) { return true; }
        }
        false
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::is_path_crossing("".to_string()), false);
    assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
}