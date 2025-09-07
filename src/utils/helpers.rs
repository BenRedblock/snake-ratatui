use crate::utils::enums::Direction;

pub fn convert_ms_to_string(ms: &u64) -> String {
    let seconds = ms / 1000;
    let minutes = seconds / 60;
    let remaining_seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, remaining_seconds)
}

pub fn get_directionvector_from_snake(snake: &Vec<(f64, f64)>) -> (f64, f64) {
    (snake[0].0 - snake[1].0, (snake[0].1 - snake[1].1))
}

pub fn get_direction_from_vector(vector: &(f64, f64)) -> Direction {
    if vector.0.abs() > vector.1.abs() {
        if vector.0 > 0.0 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        if vector.1 < 0.0 {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}
