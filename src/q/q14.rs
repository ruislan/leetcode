mod q14 {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // 方法1
        // if strs.is_empty() {
        //     return String::new();
        // }
        // let mut common_prefix = strs.get(0).unwrap().clone();
        // let mut i = 1;
        // let len = strs.len();
        // while i < len {
        //     let pending_str = strs.get(i).unwrap();
        //     let mut pending_common_prefix = String::new();
        //     let mut pending_chars = pending_str.chars();
        //     let mut common_prefix_chars = common_prefix.chars();
        //     loop {
        //         let pending_char = pending_chars.next();
        //         let common_prefix_char = common_prefix_chars.next();
        //         if None == pending_char || None == common_prefix_char {
        //             break;
        //         }
        //         if common_prefix_char == pending_char {
        //             pending_common_prefix.push(common_prefix_char.unwrap());
        //         } else {
        //             break;
        //         }
        //     }
        //
        //     common_prefix = pending_common_prefix.clone();
        //     if common_prefix.is_empty() {
        //         break;
        //     }
        //     i += 1;
        // }
        // common_prefix

        // 方法2
        if strs.is_empty() { return String::new(); }

        let mut common_prefix = strs.get(0).unwrap().clone();

        for i in 1..strs.len() {
            let mut pending_common_prefix = String::new();

            for (ch1, ch2) in strs.get(i).unwrap().chars().zip(common_prefix.chars()) {
                if ch1 == ch2 {
                    pending_common_prefix.push(ch1);
                } else {
                    break;
                }
            }

            common_prefix = pending_common_prefix.clone();

            if common_prefix.is_empty() {
                break;
            }
        }
        
        common_prefix
    }
}