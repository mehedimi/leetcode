use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    let mut num: i32 = 0;
    let mut is_odd = false;

    for char in s.chars() {
        char_map.entry(char).and_modify(|e| *e += 1).or_insert(1);
    }

    for ch in char_map.iter() {
        if ch.1 % 2 == 1 {
            if !is_odd {
                num += ch.1;
                is_odd = true
            } else {
                num += ch.1 - 1;
            }
        } else {
            num += ch.1;
        }
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn second() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn third() {
        assert_eq!(longest_palindrome("bb".to_string()), 2);
    }

    #[test]
    fn fourth() {
        assert_eq!(longest_palindrome("ccc".to_string()), 3);
    }

    #[test]
    fn fifth() {
        assert_eq!(longest_palindrome("ababababa".to_string()), 9);
    }
}
