use std::collections::HashMap;

pub fn handle(s: String) -> i32 {
    let char_map = HashMap::from([('M', 1000), ('D', 500), ('C', 100), ('L', 50), ('X', 10), ('V', 5), ('I', 1)]);
    let p_map = HashMap::from([('V', 'I'), ('X', 'I'), ('L', 'X'), ('C', 'X'), ('D', 'C'), ('M', 'C')]);
    let mut n = 0;

    let mut chars: Vec<char> = s.chars().collect();

    let mut index = 0;

    while index < chars.len() {
        let char = chars.get(index).unwrap();

        let num = char_map.get(&char).unwrap();

        if let Some(next_char) = chars.get(index + 1) {
            if let Some (next_pair_char) = p_map.get(&next_char) {
                if next_pair_char == char {
                    let next_num = char_map.get(&next_char).unwrap();
                    n = n + (next_num - num);
                    index = index + 2;
                    continue;
                }
            }
        }

        n = n + num;
        index = index + 1;
    }

    n
}