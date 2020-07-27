mod q_852 {
    #[test]
    fn test_q_852() {
        assert_eq!(1, peak_index_in_mountain_array(vec![0, 1, 0]));
        assert_eq!(1, peak_index_in_mountain_array(vec![0, 2, 1, 0]));
        assert_eq!(1, peak_index_in_mountain_array(vec![0, 3, 1, 0]));
        assert_eq!(2, peak_index_in_mountain_array(vec![0, 1, 3, 0]));
        assert_eq!(
            5,
            peak_index_in_mountain_array(vec![18, 29, 38, 59, 98, 100, 99, 98, 90])
        );
    }

    //
    // 我们把符合下列属性的数组 A 称作山脉：
    // A.length >= 3
    // 存在 0 < i < A.length - 1 使得A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]
    // 给定一个确定为山脉的数组，返回任何满足 A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1] 的 i 的值。
    //
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        for i in 0..a.len() {
            if a[i] > a[i + 1] {
                return i as i32;
            }
        }
        -1
    }
}