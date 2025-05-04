fn main() {
    println!("Hello, world!");
    Solution::example_function();
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Result: {:?}", result);
}

struct Solution;

impl Solution {
    pub fn example_function() {
        // Implement your solution here
        print!("Hello from Solution::example_function!");
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in nums.iter().enumerate().skip(i + 1) {
                if num1 + num2 == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_optimized(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = num_map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            num_map.insert(num, i);
        }
        vec![]
    }
}