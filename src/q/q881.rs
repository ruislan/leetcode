use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        // 方法1
        // 贪心
        // 首选最重的，如果还有余地，可以选择最轻的，
        // 从两边向中间走，直到所有的都被选择
        // AC 12ms 2.3mb
        let n = people.len();
        let mut people = people;
        people.sort_unstable();
        let mut answer = 0;
        let mut lo = 0;
        let mut hi = n - 1;
        while hi < n && lo <= hi {
            if people[lo] + people[hi] <= limit {
                lo += 1;
            }
            hi -= 1;
            answer += 1;
        }
        answer
    }
}
