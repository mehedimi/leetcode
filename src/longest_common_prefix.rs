pub fn handle(strs: Vec<String>) -> String {
    let mut s = String::new();
    let mut c_index = 0;
    let mut c: Option<char> = None;

    'outer: loop {
        'inner: for str in &strs {
            if c.is_none() {
                if let Some(char) = str.chars().nth(c_index) {
                    c = Some(char);
                    continue 'inner;
                } else {
                    break 'outer;
                }
            }

            if let Some(char) = str.chars().nth(c_index) {
                if char != c.unwrap() {
                    c = None;
                    break 'outer;
                }
            } else {
                break 'outer
            }
        }

        c_index = c_index + 1;
        if c.is_none() {
            break;
        } else {
            s.push(c.unwrap());
            c = None;
        }
    }

    s
}