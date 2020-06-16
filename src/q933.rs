mod q933 {
    use std::collections::VecDeque;

    struct RecentCounter {
        pings: VecDeque<i32>
    }
    
    impl RecentCounter {
        fn new() -> Self {
            RecentCounter {
                pings: VecDeque::new(),
            }
        }

        fn ping(&mut self, t: i32) -> i32 {
            while let Some(&x) = self.pings.front() {
                if x < t - 3000_i32 { self.pings.pop_front(); } else { break; }
            }
            self.pings.push_back(t);
            return self.pings.len() as i32;
        }
    }
}