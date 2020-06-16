mod q744 {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        // 方法1
        // for i in 0..letters.len() {
        //     if letters[i] > target { return letters[i]; }
        // }
        // letters[0]
        
        // 方法2
        match letters.binary_search(&target) {
            Ok(x) => {
                if x == letters.len() - 1 { letters[0] } else { letters[x + 1] }
            }
            Err(x) => {
                if x == letters.len() { letters[0] } else { letters[x] }
            }
        }
    }
}