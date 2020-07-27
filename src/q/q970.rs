mod q970 {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        // 方法1
        // let mut res = std::collections::HashSet::new();
        // let mut j = 0;
        // let a = std::cmp::min(x, y);
        // let b = std::cmp::max(x, y);
        //
        // loop {
        //     let mut i = 0;
        //     loop {
        //         let sum = a.pow(i) + b.pow(j);
        //         if sum <= bound { res.insert(sum); }
        //         if a == 1 || sum > bound { break; }
        //         i += 1;
        //     }
        //     j += 1;
        //     if b.pow(j) + 1 > bound || b == 1 { break; }
        // }
        //
        // res.drain().collect()

        // 方法2
        let mut set = std::collections::HashSet::new();

        for i in 0..20 {
            for j in 0..20 {
                let pow_int = x.saturating_pow(i).saturating_add(y.saturating_pow(j));

                if pow_int <= bound {
                    set.insert(pow_int);
                }
            }
        }

        set.drain().collect()
    }
}