use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        // 方法1
        // 将pairs转换成图，然后利用广度或者深度将所有联通的节点都归集在一起
        // 然后将节点中的字符串按照字典顺序排序
        // 然后依次将他们放到数组的位置中
        // Passed 56ms 15.7mb
        let mut graph = std::collections::HashMap::new();
        for pair in pairs {
            let a = pair[0] as usize;
            let b = pair[1] as usize;
            graph.entry(a).or_insert(Vec::new()).push(b);
            graph.entry(b).or_insert(Vec::new()).push(a);
        }

        let mut n = s.len();
        let mut s: Vec<char> = s.chars().collect();
        let mut answer = vec![' '; n];
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                visited[i] = true;
                let mut stack = vec![i];
                let mut group = vec![i];
                while !stack.is_empty() {
                    let node = stack.pop().unwrap();
                    if let Some(nodes) = graph.get(&node) {
                        for &next in nodes {
                            if !visited[next] {
                                visited[next] = true;
                                group.push(next);
                                stack.push(next);
                            }
                        }
                    }
                }

                group.sort_unstable();
                let mut sorted = group.clone();
                sorted.sort_unstable_by(|&a, &b| s[a].cmp(&s[b]));
                for i in 0..group.len() {
                    answer[group[i]] = s[sorted[i]];
                }
            }
        }

        answer.into_iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]), "bacd".to_string());
    assert_eq!(Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2], vec![0, 2]]), "abcd".to_string());
}