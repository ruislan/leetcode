mod q1221 {
    #[test]
    pub fn test_q122() {
        assert_eq!(0, balanced_string_split("R".to_string()));
        assert_eq!(0, balanced_string_split("LLR".to_string()));
        assert_eq!(4, balanced_string_split("RLRRLLRLRL".to_string()));
        assert_eq!(3, balanced_string_split("RLLLLRRRLR".to_string()));
        assert_eq!(1, balanced_string_split("LLLLRRRR".to_string()));
    }

    pub fn balanced_string_split(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut count = 0;
        for ch in s.chars() {
            if !stack.is_empty() && *stack.last().unwrap() != ch {
                stack.pop();
                if stack.is_empty() { count += 1; }
            } else {
                stack.push(ch);
            }
        }
        count
    }
}