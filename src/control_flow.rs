pub fn validate_age_restricted_access(age: i32) -> bool {
    let mut is_valid = false;
    if age <= 18 { is_valid = false; } else { is_valid = true; }
    is_valid
}
pub fn is_even(number: i32) -> bool {
    if number % 2 == 0 {
        return true;
    }
    false
}
pub fn is_odd(number: i32) -> bool {
    if number % 3 == 0 {
        return true;
    }
    false
}