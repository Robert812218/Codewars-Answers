fn index(nums: &[u64], n: usize) -> Option<u64> {
    if n >= nums.len() {
        return None;
    }
    
    let result = nums[n].pow(n as u32);
    Some(result)
}
