pub fn bubble(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                arr[i] = arr[i] ^ arr[j];
                arr[j] = arr[i] ^ arr[j];
                arr[i] = arr[i] ^ arr[j];
            }
        }
    }
    arr
}


#[test]
fn test() {
    assert_eq!(bubble(vec![3, 5, 7, 1, 2]), vec![1, 2, 3, 5, 7]);
    assert_eq!(bubble(vec![3, 5, 7, 1, 2, 2]), vec![1, 2, 2, 3, 5, 7]);
}