use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut n = 0;

    let mut map = HashSet::new();

    for num in nums {
        map.insert(num);
    }
    

    return n;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert_eq!(longest_consecutive(vec![100,4,200,1,3,2]), 4);
    }
}