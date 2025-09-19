fn main() {
    println!("Solve the problem for search insert position");
    dbg!(search_insert(vec![1, 3, 5, 6], -10000));
}
fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut low: usize = 0;
    let mut high: usize = nums.len() - 1;
    let mut mid: usize = 0;
    while low <= high {
        mid = (high + low) / 2;
        if nums[mid] == target {
            return mid.try_into().unwrap();
        } else if nums[mid] < target {
            low = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            high = mid - 1;
        }
    }

    if nums[mid] > target {
        return mid.try_into().unwrap();
    } else {
        return (mid + 1).try_into().unwrap();
    }
}
