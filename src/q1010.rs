mod q1010 {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        // 方法1
        // let mut bag = vec![0;60];
        // for i in 0..time.len() {
        //     bag[(time[i] % 60) as usize] += 1;
        // }
        //
        // let mut count = 0;
        // for i in 1..=bag[0] {
        //     count += bag[0] - i as i32;
        // }
        // for i in 1..=bag[30] {
        //     count += bag[30] - i as i32;
        // }
        // for i in 1..30 {
        //     count += bag[i] * bag[60 - i];
        // }
        // count
        // 方法2
        let mut bag = vec![0; 60];
        let mut count = 0;
        for t in time {
            let m = t % 60;
            let remain = if m == 0 { 0 } else { 60 - m };
            count += bag[remain as usize];
            bag[m as usize] += 1;
        }
        count
    }
}