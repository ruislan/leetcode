// 方法1
// 通过排序可以将数据排列好之后再找中位数就简单了
// 但是排序显然会比较耗时
// 这里通过可能是因为find_median调用较少
// AC 532ms 26.6mb
// struct MedianFinder {
//     nums: Vec<i32>
// }
// impl MedianFinder {
//
//     fn new() -> Self {
//         MedianFinder { nums: Vec::new() }
//     }
//
//     fn add_num(&mut self, num: i32) {
//         self.nums.push(num);
//     }
//
//     fn find_median(&mut self) -> f64 {
//         self.nums.sort_unstable();
//         let n = self.nums.len();
//         let mut mid = n >> 1;
//         if n & 1 == 1 {
//             self.nums[mid] as f64
//         } else {
//             (self.nums[mid] + self.nums[mid - 1]) as f64 / 2 as f64
//         }
//     }
// }

use std::cmp::Reverse;
// 方法1
// 利用两个堆，大顶堆和小顶堆
// 我们维护两个堆的数据，小顶堆存储大数据，大顶堆存储小数据
// 将奇数维护在小顶堆上
// 这样，如果是奇数直接取小顶堆的堆顶数据即可
// 如果是偶数，直接取大顶堆和小顶堆的数据除以2即可
// AC 76ms 26.9mb
use std::collections::BinaryHeap;

struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
    n: i32,
}

#[allow(unused)]
impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lo: BinaryHeap::new(),
            hi: BinaryHeap::new(),
            n: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.lo.is_empty() || num <= *self.lo.peek().unwrap() {
            self.lo.push(num);
            if self.hi.len() + 1 < self.lo.len() {
                self.hi.push(Reverse(self.lo.pop().unwrap()));
            }
        } else {
            self.hi.push(Reverse(num));
            if self.hi.len() > self.lo.len() {
                self.lo.push(self.hi.pop().unwrap().0);
            }
        }
        self.n += 1;
    }

    fn find_median(&self) -> f64 {
        if self.n & 1 == 1 {
            *self.lo.peek().unwrap() as f64
        } else {
            (*self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2 as f64
        }
    }
}