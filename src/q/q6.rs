use crate::q::Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        // 方法1
        // if num_rows < 2 { return s; }
        // let rows = num_rows as usize;
        // let mut chars = s.chars();
        // let mut metrx: Vec<Vec<char>> = vec![Vec::new(); rows];
        // let gap = rows - 2;
        // let mut ct = 0;
        // loop {
        //     let ch = chars.next();
        //     if ch == None { break; }
        //     if ct < rows {
        //         metrx[ct].push(ch.unwrap());
        //         ct += 1;
        //     } else if ct < rows + gap {
        //         metrx[2 * (rows - 1) - ct].push(ch.unwrap());
        //         ct += 1;
        //     }
        //     if ct == rows + gap { ct = 0; }
        // }
        //
        // let mut result = String::new();
        // for v in metrx {
        //     for ch in v {
        //         result.push(ch);
        //     }
        // }
        // result

        // 方法2
        // if num_rows == 1 { return s; }
        // let rows:usize = num_rows as usize;
        // let chars:Vec<char> = s.chars().collect();
        // let mut result = String::new();
        // let n = s.len();
        // let cycle_len = 2 * rows - 2;
        //
        // for i in 0..rows {
        //     let mut j = 0;
        //     loop {
        //         if j + i >= n { break; }
        //         result.push(chars[j + i]);
        //         if i != 0 && i != rows - 1 && j + cycle_len - i < n {
        //             result.push(chars[j + cycle_len - i]);
        //         }
        //         j += cycle_len;
        //     }
        // }
        // result

        // 方法3
        if num_rows == 1 { return s; }
        let rows = num_rows as usize;
        let mut metrx: Vec<Vec<char>> = vec![Vec::new(); rows];
        let mut cur_row = 0;
        let mut flip = false;
        for ch in s.chars() {
            metrx[cur_row].push(ch);
            if cur_row == 0 || cur_row == rows - 1 {
                flip = !flip;
            }
            if flip { cur_row += 1; } else { cur_row -= 1; }
        }
        metrx.concat().iter().collect::<String>()
    }
}