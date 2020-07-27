mod q1351 {
    #[test]
    fn test_q1351() {
        assert_eq!(count_negatives(vec![vec![4, 3, 2, -1], vec![3, 2, 1, -1], vec![1, 1, -1, -2], vec![-1, -1, -2, -3]]), 8);
    }

    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // 迭代grid的row和col，因为是负数的数目，同时是非递增的，所以到负数时即停，然后col的长度减去正数的数目即是负数
        // 不过我们也可以直接用流的filter来计算负数
        grid.iter().fold(0, |sum, row| sum + row.iter().filter(|&&x| x < 0).count() as i32)
    }
}