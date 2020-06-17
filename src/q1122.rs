mod q1122 {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut bag = vec![0; 1001];
        for i in 0..arr1.len() {
            bag[arr1[i] as usize] += 1;
        }
        let mut res = Vec::new();
        for i in 0..arr2.len() {
            for _ in 0..bag[arr2[i] as usize] {
                res.push(arr2[i]);
                bag[arr2[i] as usize] -= 1;
            }
        }
        for i in 0..bag.len() {
            for _ in 0..bag[i] {
                res.push(i as i32);
            }
        }
        res
    }
}