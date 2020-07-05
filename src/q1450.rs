use crate::Solution;

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        // 方法1
        // (0..start_time.len())来迭代start_time和end_time，
        // 然后filter，留下query_time in range[st[i], et[i]]
        // 然后count即为结果
        0
    }
}