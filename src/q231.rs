mod q_231 {
    #[test]
    fn test_q_231() {
        assert_eq!(false, is_power_of_two(1073741825));
    }

    pub fn is_power_of_two(n: i32) -> bool {
        let mut left = 0;
        let mut right = 30;
        if n > (1 << right) || n <= 0 {
            return false;
        }
        while left <= right {
            let mid = (right + left) / 2;
            let x = 1 << mid;
            if x == n {
                return true;
            }
            if x > n {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}