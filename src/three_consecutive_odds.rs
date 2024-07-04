pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }

    let mut index = 0;

    let l = arr.len() - 3;
    while index <= l {
        if arr.get(index).unwrap() % 2 == 0
            || arr.get(index + 1).unwrap() % 2 == 0
            || arr.get(index + 2).unwrap() % 2 == 0
        {
            index += 1;
            continue;
        }
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(three_consecutive_odds(vec![2, 6, 4, 1]), false);
    }

    #[test]
    fn second() {
        assert_eq!(
            three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true
        );
    }
}
