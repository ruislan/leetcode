mod q1200 {
    #[test]
    pub fn test_q1200() {
        assert_eq!(minimum_abs_difference(vec![4, 2, 1, 3]), vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(minimum_abs_difference(vec![1, 3, 6, 10, 15]), vec![vec![1, 3]]);
        assert_eq!(minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]), vec![vec![-14, -10], vec![19, 23], vec![23, 27]]);
    }

    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        // 方法一：先对数组进行升序排序，然后逐步找出两个相邻数之间的距离的绝对值，替换最小的那个
        // Passed 16ms, 3mb; 88.89%,100.0%
        let mut arr = arr;
        arr.sort_unstable();
        let mut min = i32::max_value();
        let mut res = Vec::new();
        for i in 1..arr.len() {
            let diff = arr[i] - arr[i - 1];
            if diff < min {
                min = diff;
                res = vec![vec![arr[i - 1], arr[i]]];
            } else if diff == min {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }
        res

        // 方法二：创建一个10^6*2+1的数组，然后将arr中的数字作为index放入数组中
        // 然后逐步找出两个大于1的index的差值，替换最小的那个
        // Passed 64ms,11.4mb; 33.33%,100.0%
        // let mut bag = vec![0; 2000001];
        // for i in 0..arr.len() {
        //     bag[(arr[i] + 1000000) as usize] = 1;
        // }
        // let mut arr = Vec::new();
        // for i in 0..bag.len() {
        //     if bag[i] > 0 {
        //         arr.push(i as i32 - 1000000);
        //     }
        // }
        //
        // let mut min = i32::max_value();
        // let mut res = Vec::new();
        // for i in 1..arr.len() {
        //     let diff = (arr[i] - arr[i - 1]).abs();
        //     if diff < min {
        //         min = diff;
        //         res = vec![vec![arr[i - 1], arr[i]]];
        //     } else if diff == min {
        //         res.push(vec![arr[i - 1], arr[i]]);
        //     }
        // }
        // res
    }
}