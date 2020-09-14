use crate::q::Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort();
        s.sort();

        let (mut p_g, mut p_s, mut count) = (0, 0, 0);
        loop {
            if p_g == g.len() || p_s == s.len() { break; }

            if s[p_s] >= g[p_g] {
                count += 1;
                p_g += 1;
                p_s += 1;
            } else {
                p_s += 1;
            }
        }

        count
    }
}