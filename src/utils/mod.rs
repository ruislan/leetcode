pub mod union_find;
pub mod segment_tree;
pub mod fenwick_tree;


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

#[allow(unused)]
pub fn gcd(d1: i32, d2: i32) -> i32 {
    if d2 == 0 { d1 } else { gcd(d2, d1 % d2) }
}