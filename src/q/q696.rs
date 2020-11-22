use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let nums = s.chars().collect::<Vec<char>>();
        let (mut ans, mut prev, mut cur) = (0, 0, 1);
        for i in 1..s.len() {
            if nums[i - 1] != nums[i] {
                ans += std::cmp::min(prev, cur);
                prev = cur;
                cur = 1;
            } else {
                cur += 1;
            }
        }
        ans + std::cmp::min(prev, cur)
    }
}