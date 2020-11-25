use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 排序points，迭代points
        // 找出区间覆盖的最小交集即可
        // 这里首先我按左边排序，就出现了个小问题，
        // 如果有覆盖最广的那个，里面那几个虽然相互无法覆盖，但是没有被计算
        // 接下来按最右边排序，完美解决
        // 这里总结一下，遇到类似这种区间的问题，我们首先就是要排序，排序无非就是按左端还是右端，就看题更适合哪一个
        // Passed 16ms 2.9mb
        if points.is_empty() { return 0; }
        let mut points = points;
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut answer = 1;
        let mut point = points[0][1];
        for range in points {
            if point < range[0] {
                point = range[1];
                answer += 1;
            }
        }
        answer
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2]]), 1);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 5], vec![2, 3]]), 1);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3]]), 1);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![2, 3], vec![2, 3]]), 1);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![-2, -1], vec![2, 3]]), 2);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 5], vec![1, 3], vec![4, 5]]), 2);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![2, 3], vec![2, 3], vec![2, 3], vec![2, 3]]), 1);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]), 2);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 2);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]), 4);
}