use serde::{Serialize, Deserialize};
use std::{env, collections::HashMap};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static RESULTS_CACHE: Lazy<Mutex<HashMap<String, Vec<AnalysisResults>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn load_env() {
    dotenv::dotenv().ok();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnalysisResults {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub total_score: f64,
    pub detailed_scores: Vec<ScoreDetail>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScoreDetail {
    pub category: String,
    pub score: f64,
}

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        load_env();

        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
        }
    }
}

pub fn fetch_analysis_results() -> Vec<AnalysisResults> {
    let mut cache = RESULTS_CACHE.lock().unwrap();

    match cache.get("analysis_results") {
        Some(results) => results.clone(),
        None => {
            let results = vec![
                AnalysisResults {
                    id: 1,
                    title: "Example Analysis 1".to_string(),
                    summary: "This is a summary of the analysis results.".to_string(),
                    total_score: 89.5,
                    detailed_scores: vec![
                        ScoreDetail { category: "Performance".to_string(), score: 90.0 },
                        ScoreDetail { category: "Accuracy".to_string(), score: 89.0 },
                    ],
                },
            ];
            cache.insert("analysis_results".to_string(), results.clone());
            results
        }
    }
}