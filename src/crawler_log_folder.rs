pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut step = 0;

    for log in logs {
        match log.as_str() {
            "../" => {
                if step > 0 {
                    step -= 1;
                }
            }
            "./" => {}
            _ => {
                step += 1;
            }
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
    }

    #[test]
    fn second() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string()
            ]),
            3
        );
    }
}
