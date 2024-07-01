pub fn is_match(s: String, p: String) -> bool {
    if p == "*" {
        return true;
    }

    let s_vec: Vec<char> = s.chars().collect();
    let p_vec: Vec<char> = p.chars().collect();
    let mut s_index = 0;
    let mut p_index = 0;

    loop {
        if let Some(next_p) = p_vec.get(p_index) {
            match next_p {
                '*' => {
                    let Some(next_s_char) = s_vec.get(s_index + 1) else {
                        return true;
                    };

                    let Some(next_p_char) = p_vec.get(p_index + 1) else {
                        return true;
                    };

                    if next_s_char == next_p_char {
                        p_index += 1;
                    }

                    s_index += 1;
                    continue;
                }
                '?' => {
                    s_index += 1;
                }
                _ => {
                    if let Some(sc) = s_vec.get(s_index) {
                        if *sc == *next_p {
                            s_index += 1;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            p_index += 1;
        } else {
            if s_vec.get(s_index).is_none() {
                return true;
            }
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test_second() {
        assert_eq!(is_match("aa".to_string(), "*".to_string()), true);
    }

    #[test]
    fn test_third() {
        assert_eq!(is_match("cb".to_string(), "?a".to_string()), false);
    }

    #[test]
    fn test_fourth() {
        assert_eq!(is_match("acdcb".to_string(), "a*c?b".to_string()), false);
    }

    #[test]
    fn test_fifth() {
        assert_eq!(is_match("adceb".to_string(), "*a*b".to_string()), true);
    }
    #[test]
    fn test_sixth() {
        assert_eq!(is_match("aa".to_string(), "aa".to_string()), true);
    }

    #[test]
    fn test_seventh() {
        assert_eq!(is_match("mississippi".to_string(), "m??*ss*?i*pi".to_string()), false);
    }
}
