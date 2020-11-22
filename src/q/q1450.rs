use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        // 方法1
        // (0..start_time.len())来迭代start_time和end_time，
        // 然后filter，留下query_time in range[st[i], et[i]]
        // 然后count即为结果
        // Passed 0ms 2.1mb
        (0..start_time.len()).filter(|&i| (start_time[i]..=end_time[i]).contains(&query_time)).count() as i32
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::busy_student(vec![], vec![], 0), 0);
    assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    assert_eq!(Solution::busy_student(vec![4], vec![4], 5), 0);
    assert_eq!(Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7), 0);
    assert_eq!(Solution::busy_student(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], vec![10, 10, 10, 10, 10, 10, 10, 10, 10], 5), 5);
}