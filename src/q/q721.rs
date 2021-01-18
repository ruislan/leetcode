use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 方法1
        // 并查集
        // 想象一下同一个accounts[i]下的自成一个集合
        // 然后找到不同accounts[i]的集合中互有的元素，
        // 然后合并集合
        // 1.由于Rust的hash算法是安全hash，性能并不高，所以这里的时间并不是最佳的
        // 2.并查集也没有做路径压缩
        // 3.还有一个优化就是给mail编号来指代字符串
        // Passed 104ms 3.9mb
        let mut parents = std::collections::HashMap::new();
        let mut mail2account = std::collections::HashMap::new();
        for i in 0..accounts.len() {
            let name = &accounts[i][0];
            for j in 1..accounts[i].len() {
                parents.entry(&accounts[i][j]).or_insert(&accounts[i][j]);
                mail2account.entry(&accounts[i][j]).or_insert(name);
            }
        }
        for i in 0..accounts.len() {
            for j in 2..accounts[i].len() {
                let mut a = &accounts[i][j - 1];
                let mut b = &accounts[i][j];
                while &a != parents.get(a).unwrap() { a = parents.get(a).unwrap(); }
                while &b != parents.get(b).unwrap() { b = parents.get(b).unwrap(); }
                if a == b { continue; }
                *parents.get_mut(b).unwrap() = a;
            }
        }

        let mut sets = std::collections::HashMap::new();
        for (&k, &v) in parents.iter() {
            let mut root = k;
            while &root != parents.get(root).unwrap() { root = parents.get(root).unwrap(); }
            sets.entry(root).or_insert(Vec::new()).push(k);
        }

        let mut answer = Vec::new();
        sets.into_iter().for_each(|(head, set)| {
            let mut row = vec![mail2account.get(head).unwrap().to_string()];
            let mut mails: Vec<String> = set.into_iter().map(|s| s.to_string()).collect();
            mails.sort_unstable();
            row.append(&mut mails);
            answer.push(row);
        });
        answer

        // 方法2
        // 先建立连通图graph，然后深度优先搜索即可
        // Passed 44ms 4.8mb
        // let mut graph = std::collections::HashMap::new();
        // let mut mail2account = std::collections::HashMap::new();
        // for i in 0..accounts.len() {
        //     let name = &accounts[i][0];
        //     for j in 1..accounts[i].len() {
        //         mail2account.entry(&accounts[i][j]).or_insert(name);
        //     }
        //
        //     for j in 2..accounts[i].len() {
        //         graph.entry(&accounts[i][j - 1]).or_insert(Vec::new()).push(&accounts[i][j]);
        //         graph.entry(&accounts[i][j]).or_insert(Vec::new()).push(&accounts[i][j - 1]);
        //     }
        // }
        //
        // let mut visited = std::collections::HashSet::new();
        // let mut answer = Vec::new();
        // for i in 0..accounts.len() {
        //     if accounts[i].len() > 0 && !visited.contains(&accounts[i][1]) {
        //         let mut stack = vec![&accounts[i][1]];
        //         let mut emails = Vec::new();
        //         while !stack.is_empty() {
        //             let email = stack.pop().unwrap();
        //             if visited.insert(email) {
        //                 emails.push(email.to_string());
        //                 if let Some(nexts) = graph.get(email) {
        //                     for next in nexts.iter() {
        //                         stack.push(next);
        //                     }
        //                 }
        //             }
        //         }
        //         emails.sort_unstable();
        //         let mut row = vec![mail2account.get(&accounts[i][1]).unwrap().to_string()];
        //         row.append(&mut emails);
        //         answer.push(row);
        //     }
        // }
        //
        // answer
    }
}