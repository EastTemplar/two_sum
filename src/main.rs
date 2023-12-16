use solution_1::two_sum;

pub mod solution_1;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let results = two_sum(nums, target);

    for result in results {
        println!("{result}");
    }
}
