mod q1346 {
    #[test]
    fn test_q1346() {
        assert_eq!(check_if_exist(vec![3, 1, 7, 11]), false);
        assert_eq!(check_if_exist(vec![10, 2, 5, 3]), true);
        assert_eq!(check_if_exist(vec![7, 1, 14, 11]), true);
        assert_eq!(check_if_exist(vec![10, 2]), false);
        assert_eq!(check_if_exist(vec![-2, 4]), false);
        assert_eq!(check_if_exist(vec![-2, -4]), true);
        assert_eq!(check_if_exist(vec![-2, 0, 4]), false);
        assert_eq!(check_if_exist(vec![-2, 0, 0]), true);
    }

    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        // 方法1
        // 将所有的数都放进map中，然后迭代每个数字，如果数字*2在map中能够找到则存在这个数
        // 注意0*2 = 0，所以0必须要有2个以上
        // Passed 0ms 2mb
        // let mut map = std::collections::HashMap::new();
        // for i in 0..arr.len() {
        //     let count = map.entry(&arr[i]).or_insert(0);
        //     *count += 1;
        // }
        // for i in 0..arr.len() {
        //     if let Some(&x) = map.get(&(arr[i] * 2)) {
        //         if arr[i] != 0 || (arr[i] == 0 && x > 1) {
        //             return true;
        //         }
        //     }
        // }
        // false
        // 方法2
        // 优化方法1，其实可以在set的时候直接查询当前数*2是否存在或者当前数是偶数，那么它的一半是否存在即可
        let mut set = std::collections::HashSet::new();
        for x in arr {
            if set.contains(&(x * 2)) || (x % 2 == 0 && set.contains(&(x / 2))) {
                return true;
            }
            set.insert(x);
        }
        false
    }
}