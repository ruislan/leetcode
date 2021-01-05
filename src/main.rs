mod q;
mod lcp;
mod offer;
mod interview;
mod sorter;

fn main() {
    println!("'abb' is palindrome? [{:?}]", is_ascii_palindrome("abb"));
    println!("'abbba' is palindrome? [{:?}]", is_ascii_palindrome("abbba"));
    println!();
    println!("{:?}", slice_to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]));
    println!("{:?}", float_to_bits_string(0.75));
    println!("{:?}", float_to_bits_string(0.125));
    println!("{:?}", float_to_bits_vec(0.75).into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>());
    println!();
    println!("{}", calculate_pi(10000000));
    println!("{:?}", group_by(&vec!["a", "b", "a", "b", "a", "a"])); // a:4, b:2
    println!();

    let mut ft = FenwickTree::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", ft.arr); // [1, 2, 3, 4, 5]
    println!("{:?}", ft.nodes); // [0,1 3 3 10 5]
    println!("sum of range(1,4) is [{:?}]", ft.sum_of_range(1, 4)); // 14
    ft.update(1, 3); // [1, 3, 3, 4, 5]
    println!("{:?}", ft.arr); // [1, 3, 3, 4, 5]
    println!("{:?}", ft.nodes); // [0,1,4,3,11,5]
    println!("sum of range(4,4) is [{:?}]", ft.sum_of_range(4, 4)); // 5
    println!();

    let mut st = SegmentTree::new(vec![1, 3, 5, 7, 9, 11]);
    println!("{:?}", st.arr); // [1,3,5,7,9,11]
    println!("{:?}", st.nodes); // [36,9,27,4,5,16,11,1,3,0,0,7,9,0,0]
    println!("sum of range(2,5) is [{:?}]", st.sum_of_range(2, 5)); // 32
    st.update(4, 6); //
    println!("{:?}", st.arr); // [1,3,5,7,6,11]
    println!("{:?}", st.nodes); // [33,9,24,4,5,13,11,1,3,0,0,7,6,0,0]
    println!();

    let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6]);
    println!("{:?}", st.arr); // [1,2,3,4,5,6]
    println!("{:?}", st.nodes); // [21,6,15,3,3,9,6,1,2,0,0,4,5,0,0]
    println!("sum of range(0,3) is [{:?}]", st.sum_of_range(0, 3)); // 10
    st.update(4, 9); //
    println!("{:?}", st.arr); // [1,2,3,4,9,6]
    println!("{:?}", st.nodes); // [25,6,19,3,3,13,6,1,2,0,0,4,9,0,0]
    println!();
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct SegmentTree {
    arr: Vec<i32>,
    nodes: Vec<i32>,
}

#[allow(unused)]
impl SegmentTree {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut st = SegmentTree { arr, nodes: vec![0; n << 2] };
        st.build_tree(0, 0, n - 1);
        st
    }

    fn sum_of_range(&self, l: usize, r: usize) -> i32 {
        self.query_tree(0, 0, self.arr.len() - 1, l, r)
    }

    fn update(&mut self, i: usize, val: i32) {
        self.arr[i] = val;
        self.update_tree(0, 0, self.arr.len() - 1, i);
    }

    fn query_tree(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i32 {
        if r < lo || hi < l {
            0
        } else if lo >= l && hi <= r {
            self.nodes[node]
        } else if lo == hi {
            self.nodes[node]
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            let left_val = self.query_tree(left_child, lo, mid, l, r);
            let right_val = self.query_tree(right_child, mid + 1, hi, l, r);
            left_val + right_val
        }
    }

    fn update_tree(&mut self, node: usize, lo: usize, hi: usize, i: usize) {
        if lo == hi {
            self.nodes[node] = self.arr[i];
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            if i <= mid {
                self.update_tree(left_child, lo, mid, i);
            } else {
                self.update_tree(right_child, mid + 1, hi, i);
            }
            self.nodes[node] = self.nodes[left_child] + self.nodes[right_child];
        }
    }

    fn build_tree(&mut self, node: usize, lo: usize, hi: usize) {
        if lo == hi {
            self.nodes[node] = self.arr[lo];
        } else {
            let mid = lo + ((hi - lo) >> 1);
            let left_child = (node << 1) + 1;
            let right_child = (node << 1) + 2;
            self.build_tree(left_child, lo, mid);
            self.build_tree(right_child, mid + 1, hi);
            self.nodes[node] = self.nodes[left_child] + self.nodes[right_child];
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct FenwickTree {
    arr: Vec<i32>,
    nodes: Vec<i32>,
}

#[allow(unused)]
impl FenwickTree {
    fn new(arr: Vec<i32>) -> Self {
        let data = vec![0; arr.len() + 1];
        let mut ft = FenwickTree { arr, nodes: data };
        (0..ft.arr.len()).for_each(|i| ft.update_tree(i + 1, ft.arr[i]));
        ft
    }

    const fn lower_bit(x: usize) -> usize {
        x & ((x - 1) ^ x)
    }

    fn update_tree(&mut self, mut i: usize, val: i32) {
        while i < self.nodes.len() {
            self.nodes[i] += val;
            i += FenwickTree::lower_bit(i);
        }
    }

    fn query_tree(&self, mut i: usize) -> i32 {
        let mut answer = 0;
        while i > 0 {
            answer += self.nodes[i];
            i -= FenwickTree::lower_bit(i);
        }
        answer
    }

    pub fn update(&mut self, i: usize, val: i32) {
        self.update_tree(i + 1, val - self.arr[i]);
        self.arr[i] = val;
    }

    pub fn sum_of_range(&self, begin: usize, end: usize) -> i32 {
        self.query_tree(end + 1) - self.query_tree(begin)
    }
}

// 将数组中相同的内容做一个频率统计
#[allow(unused)]
pub fn group_by<T: Clone + Eq + std::hash::Hash>(arr: &Vec<T>) -> std::collections::HashMap<T, i32> {
    let mut groups = std::collections::HashMap::new();
    for x in arr.iter() {
        *groups.entry(x.clone()).or_insert(0) += 1;
    }
    groups
}


// 莱布尼茨公式计算pi
// pi = 4/1 - 4/3 + 4/5 - 4/7 + 4/9 - 4/11..
#[allow(unused)]
pub fn calculate_pi(terms: i32) -> f64 {
    let mut pi = 0.0;
    let mut op = 1.0;
    let mut numerator = 4.0;
    let mut denominator = 1.0;
    for _ in 0..terms {
        pi += numerator / denominator * op;
        denominator += 2.0;
        op *= -1.0;
    }
    pi
}

// 标志位(1) - 指数位(7) - 尾数位(23)
// 注意实际尾数最前面是1
#[allow(unused)]
pub fn float_to_bits_vec(f: f32) -> Vec<bool> {
    let mut bits = vec![false; 32];
    let mut f = f.to_bits();
    for pos in 0..32 {
        bits[31 - pos] = (f & 1) == 1;
        f >>= 1;
    }
    bits
}

#[allow(unused)]
pub fn float_to_bits_string(mut f: f32) -> String {
    let mut bits = Vec::new();
    float_to_bits_vec(f).into_iter()
        .enumerate()
        .for_each(|(i, bit)| {
            if i == 1 || i == 10 { bits.push('-'); }
            bits.push(if bit { '1' } else { '0' });
        });
    bits.into_iter().collect()
}

#[allow(unused)]
pub fn is_ascii_palindrome(s: &str) -> bool {
    let (mut lo, mut hi) = (0, s.len() - 1);
    while lo < hi {
        if s[lo..=lo] != s[hi..=hi] { return false; }
        hi -= 1;
        lo += 1;
    }
    true
}

#[allow(unused)]
pub fn slice_to_string_vec(s: &[&str]) -> Vec<String> {
    s.into_iter().map(|x| x.to_string()).collect()
}