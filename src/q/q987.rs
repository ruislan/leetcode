use std::collections::HashMap;

// 方法1
// 这里只需要注意插入的值是按顺序的，所以我们存储的数据始终有序
// 然后我们将hash表的值设为(i32,String)组合的Vec
// 因为值始终有序，所以我们在get的时候可以用二分查找来解决
// AC 100ms 91.6mb
struct TimeMap {
    data: HashMap<String, Vec<(i32, String)>>,
}

#[allow(unused)]
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            data: HashMap::new()
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.data.entry(key).or_insert(Vec::new()).push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(v) = self.data.get(&key) {
            match v.binary_search_by(|probe| probe.0.cmp(&timestamp)) {
                Ok(i) => v[i].1.clone(),
                Err(i) => if i == 0 { String::new() } else { v[i - 1].1.clone() }
            }
        } else {
            String::new()
        }
    }
}