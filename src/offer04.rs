use crate::Solution;

/// 同q240
impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        // 方法1
        // 直接迭代matrix，因为有序，内部vec采用二分查找，存在target即返回true
        // 迭代完返回false

        // 方法2
        // 迭代matrix，过滤第一个小于等于target和最后一个大于等于target
        // 剩下的再进行二分查找

        false
    }
}