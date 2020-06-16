mod q680 {
    pub fn valid_palindrome(s: String) -> bool {
        let chars = s.chars().collect::<Vec<char>>();
        let mut left = 0;
        let mut right = chars.len() - 1;
        while left <= right {
            if chars[left] != chars[right] {
                return self::is_valid(&chars, left + 1, right) || self::is_valid(&chars, left, right - 1);
            }
            left += 1;
            right -= 1;
        }
        true
    }

    fn is_valid(chars: &Vec<char>, mut left: usize, mut right: usize) -> bool {
        while left <= right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}