pub fn bubble(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

pub fn selection(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        let mut min = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min] { min = j; }
        }
        arr.swap(i, min);
    }
}


#[test]
fn test() {
    let arrays = vec![
        (vec![], vec![]), (vec![1], vec![1]), (vec![-1], vec![-1]),
        (vec![1, 1, 1, 1, 1, 0], vec![0, 1, 1, 1, 1, 1]),
        (vec![3, 5, 7, -1, 2], vec![-1, 2, 3, 5, 7]),
        (vec![3, 5, 7, -1, 2, 2], vec![-1, 2, 2, 3, 5, 7]),
    ];
    for mut x in arrays.clone().into_iter() {
        bubble(&mut x.0);
        assert_eq!(x.0, x.1);
    }

    for mut x in arrays.clone().into_iter() {
        selection(&mut x.0);
        assert_eq!(x.0, x.1);
    }
}