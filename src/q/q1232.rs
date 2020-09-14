use crate::q::Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        // 方法一，求效率是否相等,斜率公式 (y - y0) / (x - x0) = (y - y1) / (x - x1)
        // 因为分母可能为0，所以变形得到 (y - y0) * (x - x1) = (y - y1) * (x - x0)
        let (x0, y0, x1, y1) = (coordinates[0][0], coordinates[0][1], coordinates[1][0], coordinates[1][1]);
        for i in 2..coordinates.len() {
            let (x, y) = (coordinates[i][0], coordinates[i][1]);
            if (y - y0) * (x - x1) != (y - y1) * (x - x0) {
                return false;
            }
        }
        true
    }
}

#[test]
pub fn test_q1232() {
    assert_eq!(true, Solution::check_straight_line(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![1, 2], vec![2, 3]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![1, 2], vec![1, 5]]));
    assert_eq!(false, Solution::check_straight_line(vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![2, 1], vec![4, 2], vec![6, 3]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![2, 4], vec![2, 8], vec![2, 5]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![2, 1], vec![5, 1], vec![6, 1]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![-3, -2], vec![-1, -2], vec![2, -2], vec![-2, -2], vec![0, -2]]));
    assert_eq!(true, Solution::check_straight_line(vec![vec![0, 1], vec![1, 3], vec![-4, -7], vec![5, 11]]));
    assert_eq!(false, Solution::check_straight_line(vec![vec![1, 1], vec![2, 2], vec![2, 0]]));
}