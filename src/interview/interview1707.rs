use crate::interview::Solution;

#[allow(unused)]
impl Solution {
    pub fn truly_most_popular(names: Vec<String>, synonyms: Vec<String>) -> Vec<String> {
        // 方法1
        // 深度优先搜索
        // 首先建立邻居图graph
        // 将名字按照深度优先搜索，也即是所有相似名字归类
        // 然后统计这些归类的名字的频率即可
        // 当然本题并查集更好，不过能深度的可以优先考虑深度
        // AC 192ms 6.9mb
        let mut nums = std::collections::HashMap::new();
        let mut graph = std::collections::HashMap::new();
        for i in 0..names.len() {
            let pair: Vec<&str> = names[i].split("(").collect();
            let num_len = pair[1].len();
            let name = pair[0].to_string();
            let num = pair[1][..num_len - 1].parse::<i32>().unwrap();
            nums.insert(name.clone(), num);
            graph.entry(name.clone()).or_insert(Vec::new());
        }

        for synonym in synonyms {
            let pair: Vec<&str> = synonym.split(",").collect();
            let n2_len = pair[1].len();
            let n1 = pair[0][1..].to_string();
            let n2 = pair[1][..n2_len - 1].to_string();
            graph.entry(n1.clone()).or_insert(Vec::new()).push(n2.clone());
            graph.entry(n2).or_insert(Vec::new()).push(n1);
        }

        let mut visited = std::collections::HashSet::new();
        let mut answer = Vec::new();
        for name in graph.keys() {
            let name = name.clone();
            if visited.contains(&name) { continue; }
            let mut sets = Vec::new();
            let mut stack = vec![name];
            let mut freq = 0;
            while !stack.is_empty() {
                let name = stack.pop().unwrap();
                if !visited.insert(name.clone()) { continue; }
                sets.push(name.clone());
                if let Some(&count) = nums.get(&name) {
                    freq += count;
                }
                let voisins = graph.get(&name).unwrap();
                for voisin in voisins {
                    stack.push(voisin.clone());
                }
            }
            if !sets.is_empty() {
                sets.sort_unstable();
                answer.push(format!("{}({})", sets[0], freq));
            }
        }
        answer
    }
}