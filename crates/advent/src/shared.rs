
pub fn lcm_of(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    lcm(nums[0], lcm_of(&nums[1..]))
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}