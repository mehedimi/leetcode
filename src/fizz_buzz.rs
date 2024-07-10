pub fn fizz_buzz(n: i32) -> Vec<String> {
    let mut v = Vec::with_capacity(n as usize);

    for i in 1..n + 1 {
        let three = i % 3 == 0;
        let five = i % 5 == 0;

        if three && five {
            v.push("FizzBuzz".to_string());
        } else if three {
            v.push("Fizz".to_string())
        } else if five {
            v.push("Buzz".to_string())
        } else {
            v.push(i.to_string())
        }
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }
}
