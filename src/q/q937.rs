use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut words = Vec::new();
        let mut nums = Vec::new();
        for log in logs {
            let mut iter = log.split_ascii_whitespace();
            let mark = iter.next().unwrap();
            let first = iter.next().unwrap();
            if first.chars().next().unwrap().is_ascii_digit() {
                nums.push(log.clone());
            } else {
                let mut c = String::from(first);
                while let Some(word) = iter.next() {
                    c.push(' ');
                    c.push_str(word);
                }
                words.push((c.clone(), mark.to_string()));
            }
        }
        words.sort_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.0.cmp(&b.0) });

        let mut res = Vec::new();
        for word in words {
            res.push(String::from(word.1 + " " + &word.0));
        }
        res.append(&mut nums);
        return res;
    }
}