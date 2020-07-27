mod q1365 {
    #[test]
    fn test_q1365() {
        // assert_eq!(smaller_numbers_than_current(vec![]), vec![]);
        assert_eq!(smaller_numbers_than_current(vec![8, 1, 2, 2, 3]), vec![4, 0, 1, 1, 3]);
        assert_eq!(smaller_numbers_than_current(vec![6, 5, 4, 8]), vec![2, 1, 0, 3]);
        assert_eq!(smaller_numbers_than_current(vec![7, 7, 7, 7]), vec![0, 0, 0, 0]);
    }

    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 暴力法，直接求出每个数比其他大的个数
        // Passed 4ms 2.1mb
        // let mut res = vec![0; nums.len()];
        // for i in 0..nums.len() {
        //     let mut count = 0;
        //     for j in 0..nums.len() {
        //         if nums[i] > nums[j] {
        //             count += 1;
        //         }
        //     }
        //     res[i] = count;
        // }
        // res

        // 方法1优雅版本
        // Passed 4ms 2mb
        // nums.iter().map(|&x| nums.iter().filter(|&&n| x > n).count() as i32).collect()

        // 方法2
        // 构建一个101长度的数组arr，将nums放入数组中
        // 计算小于当前数的前缀和存储到当前数
        // 迭代nums的每个值，然后查出arr[i]的前缀和数量
        // Passed 0ms 2mb
        let mut arr = vec![(0, 0); 101];
        nums.iter().for_each(|&x| { arr[x as usize].0 += 1; });
        (1..arr.len()).for_each(|i| { arr[i].1 = arr[i - 1].0 + arr[i - 1].1; });
        nums.iter().map(|&x| arr[x as usize].1).collect()
    }
}