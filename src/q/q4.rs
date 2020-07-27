mod q4 {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        nums1.append(&mut nums2);
        nums1.sort();
        let m = nums1.len() / 2;
        if nums1.len() % 2 == 0 {
            (nums1[m] + nums1[m - 1]) as f64 / 2.0
        } else {
            nums1[m] as f64
        }
    }
}