mod q_53 {
    #[test]
    fn test_q_53() {
        assert_eq!(5, max_sub_array(vec![3, 2, -3, -1, 1, -3, 1, -1]));
        assert_eq!(21, max_sub_array(vec![8, -19, 5, -4, 20]));
        assert_eq!(-1, max_sub_array(vec![-2, -1]));
        assert_eq!(-1, max_sub_array(vec![-2, -1, -3]));
        assert_eq!(0, max_sub_array(vec![-2, -1, -3, 0]));
        assert_eq!(-1, max_sub_array(vec![-1, -2]));
        assert_eq!(-2, max_sub_array(vec![-2]));
        assert_eq!(0, max_sub_array(vec![0]));
        assert_eq!(1, max_sub_array(vec![1]));
        assert_eq!(3, max_sub_array(vec![1, 2]));
        assert_eq!(6, max_sub_array(vec![4, 2]));
        assert_eq!(0, max_sub_array(vec![-2, -1, 0]));
        assert_eq!(6, max_sub_array(vec![-2, -1, 4, -1, 2, -2, 3, -5, 4]));
        assert_eq!(6, max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        assert_eq!(9, max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -1, 4]));
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut sum = 0;
        for num in nums {
            if sum > 0 {
                sum += num;
            } else {
                sum = num;
            }
            res = std::cmp::max(res, sum);
        }
        res
    }
}