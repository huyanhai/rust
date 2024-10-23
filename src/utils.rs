/**
 * 返回0-1直接的值
 */
pub fn clamp(num: f64, min: f64, max: f64) -> f64 {
    if num < min {
        min
    } else if num > max {
        max
    } else {
        num
    }
}
