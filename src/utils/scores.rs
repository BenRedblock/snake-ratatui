use std::fs::File;
use std::fs::read_to_string;
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub player_name: String,
    pub score: i32,
}

pub struct ScoreManager {
    scores: Vec<Score>,
}

impl ScoreManager {
    pub fn new() -> Self {
        let mut manager = ScoreManager { scores: vec![] };
        manager.load_scores();
        manager.sort_scores();
        manager
    }

    pub fn add_score(&mut self, name: String, score: i32) {
        let new_score = Score {
            player_name: name,
            score,
        };
        self.scores.push(new_score);
        self.sort_scores();
        self.save_scores();
    }

    fn save_scores(&mut self) {
        let json_data = serde_json::to_string(&self.scores).expect("Failed to serialize");
        // Write the data into the encoder
        let mut file = File::create("scores.json").expect("Failed to create file");
        file.write_all(json_data.as_bytes())
            .expect("Failed to write JSON to file");
    }

    fn load_scores(&mut self) {
        let result = match read_to_string("scores.json") {
            Ok(raw_string) => serde_json::from_str(&raw_string).unwrap_or(Vec::new()),
            Err(_err) => Vec::new(),
        };
        self.scores = result;
    }

    fn sort_scores(&mut self) {
        self.scores
            .sort_by(|a, b| (b.score - a.score).cmp(&b.score));
    }

    pub fn get_scores(&self) -> &Vec<Score> {
        return &self.scores;
    }
}
