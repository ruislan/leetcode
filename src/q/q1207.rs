mod q1207 {
    #[test]
    pub fn test_q1207() {
        assert_eq!(true, unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert_eq!(false, unique_occurrences(vec![1, 2]));
        assert_eq!(true, unique_occurrences(vec![1]));
        assert_eq!(true, unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]));
    }

    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // 方法一：创建长度为2001的数组，分别存储[-1000, 1000]，统计每个数字出现的频次
        // 再创建一个hashset，将频次作为值，如果存在该值，则表示有相同频次的了
        let mut bag = vec![0; 2001];
        for i in 0..arr.len() {
            bag[(arr[i] + 1000) as usize] += 1;
        }
        let mut freq = std::collections::HashSet::new();
        for i in 0..bag.len() {
            if bag[i] > 0 {
                if freq.contains(&bag[i]) { return false; } else { freq.insert(bag[i]); }
            }
        }
        true
    }
}