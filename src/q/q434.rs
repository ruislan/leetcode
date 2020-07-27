mod q434 {
    pub fn count_segments(s: String) -> i32 {
        // 方法1
        // let v: Vec<&str> = s.split(|c:char| c == ' ').collect();
        // let mut count = 0i32;
        // for c in v {
        //     if !c.is_empty() {
        //         count += 1;
        //     }
        // }
        // count
        // 方法2
        s.split(' ').filter(|st| !st.is_empty()).collect::<Vec<&str>>().len() as i32
    }
}