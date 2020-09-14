use crate::q::Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 方法1
        // let mut rows:Vec<Vec<i32>> = Vec::new();
        // let row_index = row_index as usize + 1;
        // for i in 1..=row_index {
        //     let mut row = vec![1;i];
        //     if i > 2 {
        //         let last_row = rows.last().unwrap();
        //         for j in 1..i - 1 {
        //             row[j] = last_row[j] + last_row[j - 1];
        //         }
        //     }
        //     if i == row_index { return row; }
        //     else { rows.push(row); }
        // }
        // vec![]
        
        // 方法2
        let mut last_row: Vec<i32> = vec![1; 1];
        let row_index = row_index as usize + 1;
        for i in 2..=row_index {
            let mut row = vec![1; i];
            if i > 2 {
                for j in 1..i - 1 {
                    row[j] = last_row[j] + last_row[j - 1];
                }
            }
            last_row = row;
        }
        last_row
    }
}