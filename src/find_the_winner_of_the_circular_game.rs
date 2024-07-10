pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut list = (1..n).collect::<Vec<i32>>();

    let mut current_index = 0;

    while list.len() > 1 {
        if current_index > list.len() - 1 {
            current_index = 0;
        }


    }

    return *list.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(find_the_winner(5, 2), 3);
    }

    #[test]
    fn second() {
        assert_eq!(find_the_winner(6, 5), 1);
    }
}