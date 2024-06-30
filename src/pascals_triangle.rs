pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
    let mut index: usize = 1;

    while index <= num_rows as usize {
        match index {
            1 | 2 => {
                v.push(Vec::from([1]).repeat(index));
            }
            _ => {
                let mut inner_v: Vec<i32> = Vec::with_capacity(index);
                inner_v.push(1);
                let mut i = 1;

                while i < (index - 1) {
                    let p_v = v.get(index - 2).unwrap();

                    let left = p_v.get(i - 1).unwrap();
                    let right = p_v.get(i).unwrap();

                    inner_v.push(left + right);
                    i += 1;
                }

                inner_v.push(1);
                v.push(inner_v);
            }
        }

        index += 1;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(generate(1), vec![vec![1],]);
    }

    #[test]
    fn test_second() {
        assert_eq!(generate(2), vec![vec![1], vec![1, 1],]);
    }

    #[test]
    fn test_third() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
