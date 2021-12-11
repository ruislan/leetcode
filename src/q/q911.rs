// 方法1
// 存储一个时序，时序对应了当前这个点的时间谁有多少票
// 当需要查找的时候，采用二分法，如果没有找到，就找到最接近的上界，
// 如果这个时刻两个人的票数相同，那么就再向前寻找，找到第一个小于对方的投票就是结果
// AC 428ms 7.9 97/97Ï
struct TopVotedCandidate {
    times: Vec<(i32, Vec<i32>)>,
    persons: Vec<i32>,
    n: usize,
}

#[allow(unused)]
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let n = *persons.iter().max().unwrap() as usize + 1;
        let mut instance = TopVotedCandidate { times: Vec::new(), persons: persons.clone(), n };
        let mut votes = vec![0; n];
        for i in 0..times.len() {
            votes[persons[i] as usize] += 1;
            instance.times.push((times[i], votes.clone()));
        }
        instance
    }

    fn q(&self, t: i32) -> i32 {
        let idx_time = match self.times.binary_search_by(|(x, _)| x.cmp(&t)) {
            Ok(i) => i as i32,
            Err(i) => i as i32 - 1
        };
        let mut cands = std::collections::HashSet::new();
        let max = *self.times[idx_time as usize].1.iter().max().unwrap_or(&0);
        for i in 0..self.n {
            if self.times[idx_time as usize].1[i] == max {
                cands.insert(i as i32);
            }
        }
        if cands.len() == 1 { return cands.into_iter().next().unwrap() as i32; }
        for i in (0..=idx_time).rev() {
            let cand = self.persons[i as usize];
            if cands.contains(&cand) { return cand; }
        }
        -1
    }
}
