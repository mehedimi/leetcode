use std::collections::HashMap;

pub fn handle(i: i32) -> String {
    let mut r = String::new();
    let mut n = i;

    let map = [1000, 500, 100, 50, 10, 5, 1];

    let c_map = HashMap::from([
        (1000, 'M'),
        (500, 'D'),
        (100, 'C'),
        (50, 'L'),
        (10, 'X'),
        (5, 'V'),
        (1, 'I'),
    ]);
    let n_map: HashMap<char, i32> = c_map.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
    let b_map = HashMap::from([
        ('X', 'I'),
        ('V', 'I'),
        ('L', 'X'),
        ('C', 'X'),
        ('D', 'C'),
        ('M', 'C'),
    ]);

    while n != 0 {
        if n >= 1000 {
            r.push('M');
            n = n - 1000;
            continue;
        }

        for item in map {
            let l = c_map.get(&item).unwrap();

            if n >= item {
                r.push(*l);
                n = n - item;
                break;
            }

            if let Some(char_pair) = b_map.get(l) {
                let n_pair = n_map.get(char_pair).unwrap();

                if n >= (item - n_pair) {
                    r.push(*char_pair);
                    r.push(*l);

                    n = n - (item - n_pair);
                    break;
                }
            }
        }
    }

    r
}
