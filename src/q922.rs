mod q922 {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        // 方法2
        let mut a = a;
        let mut even = Vec::new();
        let mut odd = Vec::new();

        for i in 0..a.len() {
            if a[i] % 2 == 0 && i % 2 != 0 { // even not in it's place
                if odd.len() > 0 { // odd not in it's place too
                    a.swap(i, odd.pop().unwrap());
                } else { // odd is right
                    even.push(i);
                }
            } else if a[i] % 2 != 0 && i % 2 == 0 { // odd not in it's place
                if even.len() > 0 { //  even not in it'ss place too
                    a.swap(i, even.pop().unwrap());
                } else { // even is right
                    odd.push(i)
                }
            }
        }

        a

        // 方法1
        // let mut even = Vec::new();
        // let mut odd = Vec::new();
        //
        // for i in 0..a.len() {
        //     if a[i] % 2 == 0 {
        //         even.push(a[i]);
        //     } else {
        //         odd.push(a[i]);
        //     }
        // }
        // let mut res = Vec::new();
        // for i in 0..odd.len() {
        //     res.push(even.pop().unwrap());
        //     res.push(odd.pop().unwrap());
        // }
        // res
    }
}