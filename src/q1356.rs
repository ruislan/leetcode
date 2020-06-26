mod q1356 {
    #[test]
    fn test_q1356() {
        assert_eq!(sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]), vec![0, 1, 2, 4, 8, 3, 5, 6, 7]);
        assert_eq!(sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]), vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]);
        assert_eq!(sort_by_bits(vec![10000, 10000]), vec![10000, 10000]);
        assert_eq!(sort_by_bits(vec![2, 3, 5, 7, 11, 13, 17, 19]), vec![2, 3, 5, 17, 7, 11, 13, 19]);
        assert_eq!(sort_by_bits(vec![10, 100, 1000, 10000]), vec![10, 100, 10000, 1000]);
    }

    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        // 方法1
        // 因为最大是10^4，所以不超过两个字节16位，取0-15的数组res,
        // 数组的每个res[i]为10001的长度的0，
        // 统计arr[i]二进制1的数量为下标，放入res[下标][arr[i]]
        // 然后flatten，再返回
        // Passed 12ms 2.5mb
        // let mut m = vec![vec![0; 10001]; 16];
        // for i in 0..arr.len() {
        //     m[arr[i].count_ones() as usize][arr[i] as usize] += 1;
        // }
        //
        // let mut res = Vec::new();
        // for i in 0..m.len() {
        //     for j in 0..m[i].len() {
        //         (0..m[i][j]).for_each(|_| {
        //             res.push(j as i32);
        //         });
        //     }
        // }
        // res

        // 方法2
        // 用vec sort来尝试
        // 0ms 2.1mb
        // let mut m = vec![std::collections::BinaryHeap::new(); 16];
        // for i in 0..arr.len() {
        //     m[arr[i].count_ones() as usize].push(std::cmp::Reverse(arr[i]));
        // }
        // let mut res = Vec::new();
        // for i in 0..m.len() {
        //     while let Some(std::cmp::Reverse(x)) = m[i].pop() {
        //         res.push(x);
        //     }
        // }
        // res

        // 方法3
        // 直接用vec的count_ones作为key来排序
        // Passed 4ms 2.1mb
        let mut arr = arr;
        arr.sort_unstable_by_key(|x| (x.count_ones(), *x));
        arr
    }
}