mod q997 {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        // 方法1
        // if n == 1 { return 1; }
        // let mut p = std::collections::HashMap::new();
        // for i in 0..trust.len() {
        //     let mut e = p.entry(&trust[i][1]).or_insert(0);
        //     *e += 1;
        // }
        //
        // let mut judge = -1;
        // for (k,v) in p {
        //     if v == (n - 1) { judge = *k; }
        // }
        //
        // for i in 0..trust.len() {
        //     if trust[i][0] == judge { return -1; }
        // }
        //
        // judge
        // 方法2
        let mut p = vec![0; n as usize];
        for t in trust {
            p[t[0] as usize - 1] -= 1;
            p[t[1] as usize - 1] += 1;
        }

        for i in 0..p.len() {
            if p[i] == (n - 1) { return i as i32 + 1; }
        }

        -1
    }
}