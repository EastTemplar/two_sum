use std::{ collections::HashMap, vec };

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen_numbers = HashMap::new();

    for i in 0..nums.len() {
        let diff = &target - nums[i];

        if seen_numbers.contains_key(&diff) {
            return vec![i as i32, seen_numbers[&diff]];
        }
        seen_numbers.insert(nums[i], i as i32);
    }

    return vec![];
}
