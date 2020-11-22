struct MyHashMap {
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyHashMap {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashMap {
            data: vec![-1; 1000000],
        }
    }

    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        self.data[key as usize] = value;
    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        self.data[key as usize]
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        self.data[key as usize] = -1;
    }
}