use std::{ collections::HashMap, vec };

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen_numbers: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let difference = target - nums[i];

        if seen_numbers.contains_key(&difference) {
            return vec![*seen_numbers.get(&difference).unwrap(), i as i32];
        } else {
            seen_numbers.insert(nums[i], i as i32);
        }
    }

    return vec![];
}
