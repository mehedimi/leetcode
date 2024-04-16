pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums;
    nums.sort();

    for (i, num) in nums.iter().enumerate() {
        if i != 0 && num == nums.get(i - 1).unwrap() {
            continue;
        }

        let mut j = i + 1;
        let mut k = nums.len() - 1;

        while j < k {
            if num + nums.get(j).unwrap() + nums.get(k).unwrap() == 0 {
                out.push(vec![*num, *nums.get(j).unwrap(), *nums.get(k).unwrap()]);
                j = j + 1;
                while j < k && nums.get(j).unwrap() == nums.get(j - 1).unwrap() {
                    j = j + 1;
                }
            } else if num + nums.get(j).unwrap() + nums.get(k).unwrap() < 0 {
                j = j + 1;
            } else {
                k = k - 1;
            }
        }
    }

    out
}
