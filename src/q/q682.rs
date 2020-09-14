use crate::q::Solution;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut scores = Vec::new();
        for op in ops {
            if &op == "C" {
                scores.pop();
            } else if &op == "D" {
                if let Some(&x) = scores.last() {
                    scores.push(x << 1);
                }
            } else if &op == "+" {
                let mut sum = 0;
                let mut count = 0;
                for i in (0..scores.len()).rev() {
                    sum += scores[i];
                    count += 1;
                    if count > 1 { break; }
                }
                scores.push(sum);
            } else {
                scores.push(op.parse::<i32>().unwrap());
            }
        }
        scores.iter().sum()
    }
}