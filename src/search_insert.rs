pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len() - 1;

    let mut mid = left + (right - left) / 2;

    while left <= right {
        mid = left + (right - left) / 2;

        if *nums.get(mid).unwrap() == target {
            return mid as i32;
        }

        if *nums.get(mid).unwrap() < target {
            left = mid + 1;
        } else {
            if mid == 0 {
                return 0;
            }
            right = mid - 1;
        }
    }

    if let Some(val) = nums.get(mid) {
        if target > *val {
            return (mid + 1) as i32;
        }
    } else {
        return 0;
    }

    return mid as i32;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 5), 2);
    }

    #[test]
    fn test_second() {
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 2), 1);
    }

    #[test]
    fn test_three() {
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 7), 4);
    }

    #[test]
    fn test_four() {
        assert_eq!(search_insert(Vec::from([1, 3, 5, 6]), 0), 0);
    }

    #[test]
    fn test_five() {
        assert_eq!(search_insert(Vec::from([1, 3]), 2), 1);
    }

    #[test]
    fn test_six() {
        assert_eq!(search_insert(Vec::from([2, 5]), 1), 0);
    }
}
