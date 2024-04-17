pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    let mut find_index: Option<i32> = None;
    let mut current_index = 0;
    let mut chars = needle.chars();
    let mut haystack_chars = haystack.chars();
    let needle_char = chars.next().unwrap();

    while current_index < haystack.len() {
        if let Some(char) = haystack_chars.next() {
            if find_index.is_none() {
                if char == needle_char {
                    find_index = Some(current_index as i32);
                }
            } else {
                if let Some(mut needle_char) = chars.next() {
                    if char != needle_char {
                        current_index = find_index.unwrap() as usize + 1;
                        find_index = None;
                        chars = needle.chars();
                        haystack_chars = haystack.chars();
                        haystack_chars.nth(current_index - 1);
                        needle_char = chars.next().unwrap();
                        continue;
                    }
                }
            }
        }

        current_index = current_index + 1;
    }

    find_index.unwrap_or(-1)
}
