use crate::q::Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // 方法1
        // 迭代每个点，
        // 利用欧几里得方程求解结果((x1-x0)^2 + (y1-y0)^2).sqrt()
        // 这里其实不用开方，因为不需要求出具体的距离
        // 由于数值限制在-10000到10000，所以没有溢出的可能
        // 取堆前K个
        // Passed 40ms 3.3mb
        // let mut answer = points.into_iter().
        //     map(|point| std::cmp::Reverse((point[0] * point[0] + point[1] * point[1], point))).
        //     collect::<std::collections::BinaryHeap<std::cmp::Reverse<(i32, Vec<i32>)>>>();
        // (0..k).map(|_| answer.pop().unwrap().0.1).collect()

        // 方法2
        // 直接排序
        // Passed 32ms 2.8mb
        let mut points = points;
        points.sort_unstable_by_key(|item| item[0] * item[0] + item[1] * item[1]);
        points.truncate(k as usize);
        points
    }
}

#[test]
fn test() {
    assert_eq!(Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1), vec![vec![-2, 2]]);
    assert_eq!(Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2), vec![vec![3, 3], vec![-2, 4]]);
}