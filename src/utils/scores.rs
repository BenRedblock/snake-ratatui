use std::env;
use std::fs::{File, create_dir_all, read_to_string};
use std::io::Write;
use std::path::PathBuf;

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
        let path = Self::get_save_file_path();
        let mut file = File::create(&path).expect(&format!("Failed to create file at {}", path));
        file.write_all(json_data.as_bytes())
            .expect("Failed to write JSON to file");
    }

    fn load_scores(&mut self) {
        let path = Self::get_save_file_path();
        let result = match read_to_string(&path) {
            Ok(raw_string) => serde_json::from_str(&raw_string).unwrap_or(Vec::new()),
            Err(_err) => Vec::new(),
        };
        self.scores = result;
    }

    fn sort_scores(&mut self) {
        self.scores.sort_by(|a, b| b.score.cmp(&a.score));
    }

    pub fn get_scores(&self) -> &Vec<Score> {
        return &self.scores;
    }

    fn get_save_file_path() -> String {
        let mut path_buf = if cfg!(target_os = "windows") {
            let appdata = env::var("APPDATA").expect("APPDATA environment variable not found");
            PathBuf::from(appdata)
        } else {
            let home = env::var("HOME").expect("HOME environment variable not found");
            PathBuf::from(home)
        };

        if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
            path_buf.push(".local/share");
        }

        path_buf.push("snake-ratatui");

        if !path_buf.exists() {
            create_dir_all(&path_buf).expect("Failed to create score directory");
        }

        path_buf.push("scores.json");

        path_buf.to_str().unwrap().to_string()
    }
}
