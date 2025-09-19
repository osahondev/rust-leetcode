use std::collections::HashMap;

fn main() {
    println!("Welcome to program that solves for Two Sum");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    dbg!(two_sum(nums, target));
}
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_hash_map = HashMap::new();
    let mut index: i32 = 0;
    for num in &nums {
        let complement: i32 = target - num;
        if nums_hash_map.contains_key(&(complement)) == false {
            nums_hash_map.insert(num, index);
        } else {
            let value = nums_hash_map.get(&(complement)).copied().unwrap_or(-1);
            return vec![index, value];
        }

        index = index + 1;
    }

    return vec![];
}
