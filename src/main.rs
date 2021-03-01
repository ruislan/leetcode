use utils::fenwick_tree::FenwickTree;
use utils::segment_tree::SegmentTree;

mod q;
mod lcp;
mod offer;
mod interview;
mod xh;
// 《高频算法实战》
mod sorter;
mod utils;

fn main() {
    println!("'abb' is palindrome? [{:?}]", utils::is_ascii_palindrome("abb"));
    println!("'abbba' is palindrome? [{:?}]", utils::is_ascii_palindrome("abbba"));
    println!();
    println!("{:?}", utils::slice_to_string_vec(&["hot", "dot", "dog", "lot", "log", "cog"]));
    println!("{:?}", utils::float_to_bits_string(0.75));
    println!("{:?}", utils::float_to_bits_string(0.125));
    println!("{:?}", utils::float_to_bits_vec(0.75).into_iter().map(|x| if x { '1' } else { '0' }).collect::<String>());
    println!();
    println!("{}", utils::calculate_pi(10000000));
    println!("{:?}", utils::group_by(&vec!["a", "b", "a", "b", "a", "a"])); // a:4, b:2
    println!();

    let mut ft = FenwickTree::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", ft.arr); // [1, 2, 3, 4, 5]
    println!("{:?}", ft.nodes); // [0,1 3 3 10 5]
    println!("sum of range(1,4) is [{:?}]", ft.sum_range(1, 4)); // 14
    ft.update(1, 3); // [1, 3, 3, 4, 5]
    println!("{:?}", ft.arr); // [1, 3, 3, 4, 5]
    println!("{:?}", ft.nodes); // [0,1,4,3,11,5]
    println!("sum of range(4,4) is [{:?}]", ft.sum_range(4, 4)); // 5
    println!();


    let mut st = SegmentTree::new(vec![1, 3, 5, 7, 9, 11]);
    println!("{:?}", st.arr); // [1,3,5,7,9,11]
    println!("{:?}", st.nodes); // [36,9,27,4,5,16,11,1,3,0,0,7,9,0,0]
    println!("sum of range(2,5) is [{:?}]", st.sum_range(2, 5)); // 32
    st.update(4, 6); //
    println!("{:?}", st.arr); // [1,3,5,7,6,11]
    println!("{:?}", st.nodes); // [33,9,24,4,5,13,11,1,3,0,0,7,6,0,0]
    println!();

    let mut st = SegmentTree::new(vec![1, 2, 3, 4, 5, 6]);
    println!("{:?}", st.arr); // [1,2,3,4,5,6]
    println!("{:?}", st.nodes); // [21,6,15,3,3,9,6,1,2,0,0,4,5,0,0]
    println!("sum of range(0,3) is [{:?}]", st.sum_range(0, 3)); // 10
    st.update(4, 9); //
    println!("{:?}", st.arr); // [1,2,3,4,9,6]
    println!("{:?}", st.nodes); // [25,6,19,3,3,13,6,1,2,0,0,4,9,0,0]
    println!();
}
