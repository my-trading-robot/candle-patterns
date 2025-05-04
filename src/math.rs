pub fn calc_points_tolerance(digits: u32, points_tolerance: u32) -> f64 {
    let point_tolerance = calc_points_from_accuracy(digits) * (points_tolerance as f64) ; 
    point_tolerance
}

pub fn calc_points_from_accuracy(digits: u32) -> f64 {
    f64::powi(10.0, - (digits as i32)) 
}

pub fn round_to_precision(value: f64, digits: u32) -> f64 {
    let factor = 10f64.powi(digits as i32);
    (value * factor).round() / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_point_tolerance() {
        let result = calc_points_tolerance(2, 5); // 10^-2 * 5 = 0.05
        assert!((result - 0.05).abs() < f64::EPSILON);

        let result = calc_points_tolerance(3, 1); // 10^-3 * 1 = 0.001
        assert!((result - 0.001).abs() < f64::EPSILON);

        let result = calc_points_tolerance(0, 10); // 10^0 * 10 = 10.0
        assert!((result - 10.0).abs() < f64::EPSILON);
    }
}