/// Given an array coordinates, coordinates[i] = [x, y], where [x, y] represents the coordinate of a point,
/// Check if these points make a straight line in the XY plane.
pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    let (a, c) = (&coordinates[0], &coordinates[1]);
    let (x_diff, y_diff) = (c[0] - a[0], c[1] - a[1]);
    for b in coordinates.iter().skip(2) {
        let cross = y_diff * (b[0] - a[0]) - x_diff * (b[1] - a[1]);
        if cross.abs() > 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true() {
        assert!(check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]))
    }
    #[test]
    fn test_false() {
        assert!(!check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]))
    }
    #[test]
    fn test_corner() {
        assert!(check_straight_line(vec![vec![1, 1], vec![4, 5]]))
    }
}
