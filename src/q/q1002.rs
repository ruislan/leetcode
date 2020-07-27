mod q1002 {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        // 方法1
        // let mut map = std::collections::HashMap::new();
        // for ch in a[0].chars() {
        //     let mut count = map.entry(ch).or_insert(0);
        //     *count += 1;
        // }
        //
        // for i in 1..a.len() {
        //     let mut intra = Vec::new();
        //     for ch in a[i].chars() {
        //         if let Some(x) = map.get_mut(&ch) {
        //             *x -= 1;
        //             intra.push(ch);
        //             if *x == 0 { map.remove(&ch); }
        //         }
        //     }
        //     map.clear();
        //     for ch in intra {
        //         let mut count = map.entry(ch).or_insert(0);
        //         *count += 1;
        //     }
        // }
        //
        // let mut res = Vec::new();
        // for (k, v) in map {
        //     for i in 0..v {
        //         res.push(k.to_string());
        //     }
        // }
        // return res;
        
        // 方法2
        let mut bags = Vec::new();

        for i in 0..a.len() {
            let mut bag = vec![0; 26];
            for ch in a[i].chars() {
                bag[ch as usize - 'a' as usize] += 1;
            }
            bags.push(bag);
        }

        let mut res = Vec::new();
        for i in 0..26 {
            let mut min = bags.len();
            for bag in bags.iter() {
                if min > bag[i] { min = bag[i]; }
            }
            for _ in 0..min {
                res.push(((i as u8 + 'a' as u8) as char).to_string());
            }
        }
        return res;
    }
}