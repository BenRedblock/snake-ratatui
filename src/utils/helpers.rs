pub fn vec_to_string(vec: &Vec<(f64, f64)>) -> String {
    let mut result = String::new();
    for (x, y) in vec {
        result.push_str(&format!("({}, {}) ", x, y));
    }
    result
}

pub fn convert_ms_to_string(ms: &u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}
