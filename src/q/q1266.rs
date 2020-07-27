mod q1266 {
    #[test]
    fn test_q1266() {
        assert_eq!(7, min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]));
        assert_eq!(5, min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]));
        assert_eq!(10, min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2], vec![3, 2]]));
        assert_eq!(1004, min_time_to_visit_all_points(vec![vec![-2, 0], vec![-2, 2], vec![-2, -1000]]));
    }

    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        // 方法一：分别计算每两个点之间的x1-x0和y1-y0，取最大的那个值，然后所有的值加起来即是所需花费的最小时间
        // let mut min_time = 0;
        // for i in 1..points.len() {
        //     min_time += std::cmp::max((points[i][0] - points[i - 1][0]).abs(), (points[i][1] - points[i - 1][1]).abs());
        // }
        // min_time
        // 方法一的graceful写法
        points.windows(2).map(|x| std::cmp::max((x[1][0] - x[0][0]).abs(), (x[1][1] - x[0][1]).abs())).sum()
    }
}