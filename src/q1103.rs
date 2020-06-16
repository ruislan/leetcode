mod q1103 {
    pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
        let mut res = vec![0; num_people as usize];
        let mut n = 1;
        while candies > 0 {
            for i in 0..res.len() {
                if n > candies {
                    res[i] += candies;
                    candies = 0;
                    break;
                } else {
                    res[i] += n;
                    candies -= n;
                    n += 1;
                }
            }
        }
        res
    }
}