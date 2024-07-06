pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    let mut i = 0;

    while i <= (haystack.len() - needle.len()) {
        if let Some(p) = haystack.get(i..needle.len() + i) {
            if p == needle {
                return i as i32;
            }
            i += 1;
        } else {
            return -1;
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn second() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn third() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    }

    #[test]
    fn fourth() {
        assert_eq!(str_str("abc".to_string(), "c".to_string()), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(str_str("mississippi".to_string(), "pi".to_string()), 9);
    }
}
