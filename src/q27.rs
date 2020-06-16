mod q_27 {
    #[test]
    fn test_q_27() {
        assert_eq!(0, remove_element(&mut vec![], 5));
        assert_eq!(0, remove_element(&mut vec![3], 3));
        assert_eq!(1, remove_element(&mut vec![3], 1));
        assert_eq!(2, remove_element(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(2, remove_element(&mut vec![3, 2, 2, 3], 2));
        assert_eq!(4, remove_element(&mut vec![3, 2, 2, 3], 0));
        assert_eq!(5, remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2));
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}