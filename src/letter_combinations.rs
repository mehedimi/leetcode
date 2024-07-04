use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }

    let map = HashMap::from([
        ('2', ("abc")),
        ('3', ("def")),
        ('4', ("ghi")),
        ('5', ("jkl")),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]);

    let v = vec![];

    for char in digits.chars().collect::<Vec<_>>() {
        for i in 0..digits.len() {}
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
