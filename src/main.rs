use std::f64::consts::PI;

fn normalize(radians: f64) -> f64 {
    let mut result = radians.rem_euclid(2.0 * PI);
    if result > PI {
        result -= 2.0 * PI;
    }
    result
}

fn main() {
    for radians in (0..=(3.0 * PI * 10.0) as i32).map(|i| i as f64 * 0.1) {
        let output = normalize(radians);
        println!("{:.1}: {:.6}", radians, output);
    }
}
