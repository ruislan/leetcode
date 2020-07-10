use crate::Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        // 方法1
        // 构建一个hashset，和初始点p(0,0)，并将初始点放入set
        // 迭代path，N则p.0 += 1，S则p.0 -=1，E则p.1 += 1，W则p.1 -=1
        // 如果这个计算后的点p在set中，则返回true
        // 循环结束，返回false
        false
    }
}