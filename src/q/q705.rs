/// 方法1
// struct MyHashSet {
//     data:Vec<i32>,
// }
//
//
// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl MyHashSet {
//
//     /** Initialize your data structure here. */
//     fn new() -> Self {
//         MyHashSet {
//             data: vec![0;1000000],
//         }
//     }
//
//     fn add(&mut self, key: i32) {
//         self.data[key as usize] = 1;
//     }
//
//     fn remove(&mut self, key: i32) {
//         self.data[key as usize] = 0;
//     }
//
//     /** Returns true if this set contains the specified element */
//     fn contains(&self, key: i32) -> bool {
//         self.data[key as usize] == 1
//     }
// }

/// 方法2
struct MyHashSet {
    data: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl MyHashSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            data: vec![vec![0; 1000]; 1000],
        }
    }

    fn add(&mut self, key: i32) {
        let j = key % 1000;
        let i = key / 1000;
        self.data[i as usize][j as usize] = 1;
    }

    fn remove(&mut self, key: i32) {
        let j = key % 1000;
        let i = key / 1000;
        self.data[i as usize][j as usize] = 0;
    }

    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        let j = key % 1000;
        let i = key / 1000;
        self.data[i as usize][j as usize] == 1
    }
}