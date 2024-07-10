pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();

    for (index, word) in words.iter().enumerate() {
        if word.contains(x) {
            out.push(index as i32);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            find_words_containing(vec!["leet".to_string(), "code".to_string()], 'e'),
            vec![0, 1]
        );
    }
}
