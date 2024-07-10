pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut points: Vec<i32> = Vec::with_capacity(3);

    for op in operations {
        match op.as_str() {
            "C" => {
                points.pop().expect("Nothing in the points");
            }
            "D" => {
                points.push(points.last().unwrap() * 2);
            }
            "+" => {
                points.push(points.get(points.len() - 2).unwrap() + points.last().unwrap());
            }
            num => points.push(num.parse::<i32>().unwrap()),
        }
    }

    points.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        );
    }
}
