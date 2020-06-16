mod q28 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // 方法1
        // let s = haystack.find(&needle);
        // match s {
        //     Some(x) => x as i32,
        //     None => -1,
        // }

        // 方法2
        // let (h_len, n_len) = (haystack.len(), needle.len());
        // if n_len < 1 { return 0; }
        // let haystack: Vec<char> = haystack.chars().collect();
        // let needle: Vec<char> = needle.chars().collect();
        // for i in 0..h_len {
        //     if h_len - i < n_len { return -1; }
        //     for j in 0..n_len {
        //         if haystack[i + j] != needle[j] { break; } else if j == n_len - 1 { return i as i32; }
        //     }
        // }
        // -1

        // 方法3
        if needle.is_empty() { return 0; }
        for i in 0..haystack.len() {
            if haystack.len() - i < needle.len() { return -1; }
            if haystack[i..(i + needle.len())] == needle[0..needle.len()] { return i as i32; }
        }
        -1
    }
}