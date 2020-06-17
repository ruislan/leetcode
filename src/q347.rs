mod q347 {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 方法1
        // let mut map = std::collections::HashMap::new();
        // for n in nums {
        //     *map.entry(n).or_insert(0) += 1;
        // }
        //
        // let mut vs:Vec<(i32, i32)> = Vec::new();
        // for (k, v) in map.iter() {
        //     vs.push((*k, *v));
        // }
        // vs.sort_by(|a, b| b.1.cmp(&a.1));
        //
        // let mut res = Vec::new();
        // for i in 0..k {
        //     res.push(vs[i as usize].0);
        // }
        // res
        
        // 方法2
        let mut map = std::collections::HashMap::new();
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut vs = std::collections::BinaryHeap::new();
        for (k, v) in map.iter() {
            vs.push((*v, *k));
        }

        let mut res = Vec::new();
        for _ in 0..k {
            res.push(vs.pop().unwrap().1);
        }
        res
    }
}