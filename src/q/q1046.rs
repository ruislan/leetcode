mod q1046 {
    use std::collections::BinaryHeap;

    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut sts = stones.iter().cloned().collect::<BinaryHeap<i32>>();
        while sts.len() > 1 {
            let st1 = sts.pop().unwrap();
            let st2 = sts.pop().unwrap();
            let st = st1 - st2;
            if st != 0 { sts.push(st.abs()); }
        }
        if let Some(x) = sts.pop() {
            return x;
        }
        0
    }
}