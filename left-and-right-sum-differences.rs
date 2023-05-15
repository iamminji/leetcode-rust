impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum = Vec::new();

        left_sum.push(0);
        let mut prev = 0;
        for i in 1..nums.len() {
            left_sum.push(prev + nums[i - 1]);
            prev += nums[i - 1];
        }

        let mut result = Vec::new();
        prev = 0;
        let mut i = nums.len() - 1;
        for num in nums.iter().rev() {
            result.push((left_sum[i] - prev).abs());
            prev += nums[i];
            i -= 1;
        }
        result.reverse();
        result
    }
}
