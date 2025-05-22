pub fn vec_to_string(vec: &Vec<(f64, f64)>) -> String {
    let mut result = String::new();
    for (x, y) in vec {
        result.push_str(&format!("({}, {}) ", x, y));
    }
    result
}
