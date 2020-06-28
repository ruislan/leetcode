mod q1380 {
    struct Solution {}

    impl Solution {
        pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
            // 方法1
            // 注意所有值都不相同，所以行扫描找出最小值，存入hashsets，
            // 然后列扫描找出最大值，只要sets中有，即为幸运数
            // Passed 0ms 2mb
            let mut sets = std::collections::HashSet::new();
            (0..matrix.len()).for_each(|row| { sets.insert(*matrix[row].iter().min().unwrap_or(&0)); });
            (0..matrix[0].len())
                .map(|col| {
                    let max = (0..matrix.len()).map(|row| matrix[row][col]).max().unwrap_or(0);
                    if sets.contains(&max) { max } else { 0 }
                }).filter(|&num| num > 0)
                .collect()
        }
    }

    #[test]
    fn test_q1380() {
        assert_eq!(Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]), vec![15]);
        assert_eq!(Solution::lucky_numbers(vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]]), vec![12]);
        assert_eq!(Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
        assert_eq!(Solution::lucky_numbers(vec![vec![1, 2]]), vec![1]);
    }
}