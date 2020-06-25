mod q1332 {
    #[test]
    fn test_q1332() {
        assert_eq!(remove_palindrome_sub("ababa".to_string()), 1);
        assert_eq!(remove_palindrome_sub("abb".to_string()), 2);
        assert_eq!(remove_palindrome_sub("baabb".to_string()), 2);
        assert_eq!(remove_palindrome_sub("baabbaa".to_string()), 2);
        assert_eq!(remove_palindrome_sub("bbaabaaa".to_string()), 2);
        assert_eq!(remove_palindrome_sub("".to_string()), 0);
    }

    pub fn remove_palindrome_sub(s: String) -> i32 {
        // 方法1
        // 如果是空字符串则返回0
        // 检查该字符串是不是回文，是回文则增加1 否则返回2
        // 因为输入只会有两个字符a,b，所以我们可以直接删除子序列a或者b（注意是子序列，不是子串，子序列不需要连续，所以我们只需要删除同一个字符串即可）
        // 一种字符组成子序列总是回文，所以，最大就是2
        // 这道题关键在于审题，题目中的坑坑死人
        // Passed 0ms 2mb
        if s.is_empty() { 0 } else if s == s.chars().rev().collect::<String>() { 1 } else { 2 }
    }
}