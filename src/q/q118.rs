use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows: Vec<Vec<i32>> = Vec::new();
        let num_rows = num_rows as usize;
        for i in 1..=num_rows {
            let mut row: Vec<i32> = vec![1; i];
            if i > 2 {
                let last_row = rows.last().unwrap();
                for j in 1..(i - 1) {
                    row[j] = last_row[j - 1] + last_row[j];
                }
            }
            rows.push(row);
        }
        rows
    }
}