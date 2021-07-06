use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 方法1
        // 迭代orders：
        //   按照orders[i][1]聚合在一起
        use std::collections::{HashMap, HashSet};

        let mut tables = vec![HashMap::new(); 501];
        let mut titles = HashSet::new();
        for i in 0..orders.len() {
            let tn = orders[i][1].parse::<usize>().unwrap();
            let fi = orders[i][2].clone();
            let food_map = &mut tables[tn];
            *food_map.entry(fi.clone()).or_insert(0) += 1;
            titles.insert(fi);
        }
        let mut titles: Vec<String> = titles.into_iter().collect();
        titles.sort_unstable();
        let mut answer = vec![vec![String::from("Table")]];
        answer[0].append(&mut titles);
        for i in 1..501 {
            if !tables[i].is_empty() {
                let mut item = vec![i.to_string()];
                for j in 1..answer[0].len() {
                    let count = tables[i].get(&answer[0][j]).unwrap_or(&0);
                    item.push(count.to_string());
                }
                answer.push(item);
            }
        }
        answer
    }
}

