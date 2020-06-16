mod q1160 {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut bag = vec![0; 26];
        for b in chars.bytes() {
            bag[b as usize - 97] += 1;
        }
        let mut count = 0;
        for word in words {
            let mut dic = bag.clone();
            let mut sum = 0;
            println!("word is {}", &word);
            for w in word.bytes() {
                println!("sum is {}, w is {}, c is {}", sum, w as char, dic[w as usize - 97]);
                if dic[w as usize - 97] > 0 {
                    sum += 1;
                    dic[w as usize - 97] -= 1
                } else {
                    sum = 0;
                    break;
                }
            }
            println!("sum is {}", sum);
            count += sum;
            println!();
        }
        count
    }
}