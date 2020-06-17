mod q_849 {
    #[test]
    fn test_q_849() {
        assert_eq!(2, max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]));
        assert_eq!(3, max_dist_to_closest(vec![1, 0, 0, 0]));
        assert_eq!(1, max_dist_to_closest(vec![0, 1]));
        assert_eq!(1, max_dist_to_closest(vec![1, 0]));
        assert_eq!(1, max_dist_to_closest(vec![1, 0, 1]));
    }

    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let (mut dist_s, mut dist_m) = (0_i32, 0_i32);
        let mut last_i = -1;
        for i in 0..seats.len() {
            if 1 == seats[i] {
                if last_i == -1 {
                    dist_s = i as i32;
                } else {
                    dist_m = std::cmp::max(dist_m, (i as i32 - last_i) / 2);
                }
                last_i = i as i32;
            }
        }
        let dist_e = seats.len() as i32 - 1 - std::cmp::max(0, last_i);
        std::cmp::max(dist_m, std::cmp::max(dist_s, dist_e))
    }
}
