use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        // 方法1
        // 迭代paths，将paths[i][0]存储到hashset中
        // 迭代paths，如果paths[i][1]不存在在hashset中，即为终点
        // Passed 4ms 2.1mb
        let sets: std::collections::HashSet<&String> = paths.iter().map(|x| &x[0]).collect();
        paths.iter().find(|x| !sets.contains(&x[1])).unwrap()[1].clone()
        // for i in 0..paths.len() {
        //     if !sets.contains(&paths[i][1]) {
        //         return paths[i][1].clone();
        //     }
        // }
        // String::new()
    }
}

#[test]
fn test() {
    // assert_eq!(Solution::dest_city(vec![]), "".to_string());
    assert_eq!(Solution::dest_city(vec![vec!["London".to_string(), "New York".to_string()], vec!["New York".to_string(), "Lima".to_string()], vec!["Lima".to_string(), "Sao Paulo".to_string()]]), "Sao Paulo".to_string());
    assert_eq!(Solution::dest_city(vec![vec!["B".to_string(), "C".to_string()], vec!["D".to_string(), "B".to_string()], vec!["C".to_string(), "A".to_string()]]), "A".to_string());
    assert_eq!(Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]]), "Z".to_string());
}