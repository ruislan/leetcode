use crate::q::Solution;

#[allow(unused)]
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn fill_point(image: &mut Vec<Vec<i32>>, sr: i32, sc: i32, color: i32, new_color: i32) {
            if sr >= 0
                && sc >= 0
                && sr < image.len() as i32
                && sc < image[0].len() as i32
                && image[sr as usize][sc as usize] == color
            {
                image[sr as usize][sc as usize] = new_color;
                fill_point(image, sr - 1, sc, color, new_color);
                fill_point(image, sr + 1, sc, color, new_color);
                fill_point(image, sr, sc + 1, color, new_color);
                fill_point(image, sr, sc - 1, color, new_color);
            }
        }

        let mut image = image;
        let color = image[sr as usize][sc as usize];
        if color == new_color {
            return image;
        }
        fill_point(&mut image, sr, sc, color, new_color);
        image
    }
}

#[test]
pub fn test_q733() {
    assert_eq!(
        vec![vec![0, 0, 0], vec![0, 1, 1]],
        Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 1, 1]], 1, 1, 1)
    );
    assert_eq!(
        vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
        Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2)
    );
}