mod q1184 {
    /// 环线只需要求出右行S0和左行S1中最小的那个
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (mut sum, mut sum0) = (0, 0);
        let s = std::cmp::min(start, destination) as usize;
        let d = std::cmp::max(start, destination) as usize;

        for i in 0..distance.len() {
            sum += distance[i];
            if i >= s && i < d { sum0 += distance[i]; }
        }

        std::cmp::min(sum - sum0, sum0)
    }
}