pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut d = digits;
    let mut index = d.len() - 1;

    loop {
        let current = d.get(index).unwrap();
        // 99
        match *current {
            9 => {
                if index == 0 {
                    d[index] = 0;
                    d.insert(0, 1);
                    break;
                }

                let prev_number = d.get(index - 1).unwrap();

                match *prev_number {
                    9 => {
                        d[index] = 0;
                        index -= 1;
                        continue;
                    }
                    _ => {
                        d[index] = 0;
                        d[index - 1] += 1;
                        break;
                    }
                }
            }
            _ => {
                d[index] = *current + 1;
                break;
            }
        }
    }

    d
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_second() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_third() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn test_fourth() {
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    fn test_fifth() {
        assert_eq!(plus_one(vec![1, 9, 9]), vec![2, 0, 0]);
    }
}
