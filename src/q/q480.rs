use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        // 方法1
        // 暴力解法
        // 将每个窗口内的数字拿出来进行排序处理，找到中位数
        // 这个必然会超时 O(n * (k + klogk))
        // let n = nums.len();
        // let k = k as usize;
        // let is_odd = k & 1 == 1;
        // let mid = (k - 1) >> 1;
        //
        // let mut window = std::collections::VecDeque::new();
        // let mut answer = Vec::new();
        // for i in 0..n {
        //     if window.len() < k {
        //         window.push_back(nums[i]);
        //     }
        //     if window.len() == k {
        //         let mut vec: Vec<&i32> = window.iter().collect();
        //         vec.sort_unstable();
        //         answer.push(if is_odd {
        //             *vec[mid] as f64
        //         } else {
        //             (*vec[mid] as f64 + *vec[mid + 1] as f64) / 2_f64
        //         });
        //         window.pop_front();
        //     }
        // }
        // answer

        // 方法2
        // 改进方法1
        // 不是每次排序，而是进行局部排序
        // 我们用新进入的数字替换掉要删除的数字
        // 因为window已经有序，所有我们可以二分查找到该数字，
        // 然后替换成新的数字，再排序
        // 虽然复杂度O(n * (logk + klogk))与方法1相同
        // 但是在局部有序的情况下，排序是非常高效的
        // AC 40ms 2.1mb
        // fn get_median(window: &Vec<i32>, is_odd: bool, mid: usize) -> f64 {
        //     if is_odd {
        //         window[mid] as f64
        //     } else {
        //         (window[mid] as f64 + window[mid + 1] as f64) / 2_f64
        //     }
        // }
        //
        // let n = nums.len();
        // let k = k as usize;
        // let is_odd = k & 1 == 1;
        // let mid = (k - 1) >> 1;
        //
        // let mut window = Vec::new();
        //
        // for i in 0..k { window.push(nums[i]); }
        // window.sort_unstable();
        //
        // let mut answer = Vec::new();
        // answer.push(get_median(&window, is_odd, mid));
        //
        // for i in k..n {
        //     let j = window.binary_search(&nums[i - k]).unwrap();
        //     window[j] = nums[i];
        //     window.sort_unstable();
        //     answer.push(get_median(&window, is_odd, mid));
        // }
        //
        // answer

        // 方法3
        // 利用小顶堆large和大顶堆small分别存储偏大的数字和偏小的数字
        // 同时保证大顶堆的数量比小顶堆多1，这样的好处是
        // 如果是k是奇数，那么我们可以直接取大顶堆的堆顶即可
        // 如果是k时偶数，那么我们可以取两个堆的堆顶相加除以2即可
        // 这里都不是太难，难点在于如何维护这两个堆，
        // 假设我们的理想状态就是small和large的数量相等或者small比large多一个（这取决于窗口的奇偶）
        // 现在我们来滑动窗口，显然需要删除和插入堆中的元素
        // 由于堆不能直接删除除堆顶以外的元素，所以我们需要一个辅助的Hashmap
        // 数据结构del来存储待删除的元素，
        // 也即是说我们不能立刻删除掉数据，那么如何得知两边堆是达到了理想状态的？
        // 所以这里我们设置一个为balance的变量，初始为0表示平衡，可以将它看成天平，
        // 正的表示large重，负的表示small重，
        //     如果large添加了数据，就balance += 1
        //     如果small添加了数据，就balance -= 1
        //     如果large删除了数据，就balance -= 1
        //     如果small删除了数据，就balance += 1
        // 当我们要删除数据的时候：
        //     这个时候我们将删除的元素x与small.peek()比较，如果x > small.peek()，
        //     那么说明要删除的这个数据在large中，由于我们不能直接删除除堆顶的堆中数据，
        //     所以我们先记下这个待删除数据到del中，并且由于large删除了数据，所以两边的
        //     平衡被打破，也即是large少了一个数字，根据上述平衡应该是balance -= 1
        //     反之如果 x <= small.peek()，也就是删除的在small中，balance += 1
        // 当我们要插入数据的时候：
        //     这个时候我们要添加的元素x与small.peek()比较，如果x > small.peek()，
        //     那么说明要添加的这个数据应该放到large中，也即是balance += 1
        //     反之如果x <= small.peek()，也就是添加的在small中，balance -= 1
        // 现在问题来了，因为有可能我们删除了large，然后添加到了small，
        // balance有可能是-2和2，这不是理想状态，所以我们要调整到这个理想状态balance = 0
        // 如何调整呢？
        // 简单的来说就是谁重消谁，如果balance = 2，那么large.pop()，然后放到small中
        // 反之balance = -2, 那么small.pop()，然后放到large中
        // 这样此消彼长的情况下，balance 又等于 0了
        // 平衡之后就该还债了，分别检查两边的堆顶，如果是待删除的，就全部弹出去
        // AC 8ms 2.2mb
        use std::collections::{BinaryHeap, HashMap};
        use std::cmp::Reverse;

        let n = nums.len();
        let k = k as usize;
        let is_odd = k & 1 == 1;
        let mut large = BinaryHeap::new();
        let mut small = BinaryHeap::new();
        let mut del = HashMap::new();

        // init
        for i in 0..k { small.push(nums[i]); }
        for _ in 0..(k >> 1) {
            let x = small.pop().unwrap();
            large.push(Reverse(x));
        }

        fn get_median(is_odd: bool, small: &BinaryHeap<i32>, large: &BinaryHeap<Reverse<i32>>) -> f64 {
            if is_odd {
                *small.peek().unwrap() as f64
            } else {
                (*small.peek().unwrap() as f64 + large.peek().unwrap().0 as f64) / 2_f64
            }
        }

        let mut answer = Vec::new();
        answer.push(get_median(is_odd, &small, &large));

        // sliding window
        for i in k..n {
            let remove = nums[i - k];
            let add = nums[i];
            *del.entry(remove).or_insert(0) += 1;

            let mut balance = 0;

            // remove
            if remove <= *small.peek().unwrap() {
                balance += 1;
            } else {
                balance -= 1;
            }

            // add
            if add <= *small.peek().unwrap() {
                small.push(add);
                balance -= 1;
            } else {
                large.push(Reverse(add));
                balance += 1;
            }

            // make balance
            if balance > 0 {
                let x = large.pop().unwrap().0;
                small.push(x);
            }
            if balance < 0 {
                let x = small.pop().unwrap();
                large.push(Reverse(x));
            }

            // del
            while let Some(x) = small.peek() {
                let freq = del.entry(*x).or_insert(0);
                if *freq > 0 {
                    small.pop();
                    *freq -= 1;
                } else {
                    break;
                }
            }

            while let Some(x) = large.peek() {
                let freq = del.entry(x.0).or_insert(0);
                if *freq > 0 {
                    large.pop();
                    *freq -= 1;
                } else {
                    break;
                }
            }

            answer.push(get_median(is_odd, &small, &large));
        }

        answer
    }
}