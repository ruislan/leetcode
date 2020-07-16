use crate::Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // 方法1
        // 如果s和t的长度不同，则直接返回false
        // 将s和t按照(char => indexes)的方式分别存储到hashmap中，为map_s，map_t
        // 再创建长度为s.len()的数组arr_s和arr_t，迭代map_s和map_t，按照indexes的第一个数字的位置存储到arr的位置上
        // 再判断arr_s和arr_t是否相等
        false
    }
}