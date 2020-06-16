mod q929 {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for email in emails.iter() {
            let mut s = String::new();
            let chars: Vec<char> = email.chars().collect();
            let mut p_at = -1i32;
            let mut p_plus = -1i32;
            for i in 0..chars.len() {
                if p_at > -1 {
                    s.push(chars[i]);
                    continue;
                }

                if '@' == chars[i] {
                    p_at = i as i32;
                    p_plus = -1;
                    s.push(chars[i]);
                    continue;
                }

                if p_plus > -1 {
                    continue;
                }

                if '+' == chars[i] {
                    p_plus = i as i32;
                    continue;
                }

                if '.' != chars[i] {
                    s.push(chars[i]);
                }
            }
            set.insert(s);
        }
        set.len() as i32
    }
}