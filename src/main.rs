mod q;
mod lcp;
mod offer;
mod interview;
mod sorter;


fn main() {
    println!("{:?} is palindrome", is_ascii_palindrome("abb"));
    println!("{:?} is palindrome", is_ascii_palindrome("abbba"));
    println!("{:?}", slice_to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]));
    println!("{:?}", float_to_bits_string(0.75));
    println!("{:?}", float_to_bits_string(0.125));
    println!("{:?}", float_to_bits_vec(0.75).into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>());
    println!("{}", calculate_pi(100000000));
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
    while lo <= hi {
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