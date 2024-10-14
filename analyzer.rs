use std::env;
use std::fs;
use std::path::Path;
use std::process;

extern crate dotenv;

struct MetricResult {
    name: String,
    score: f64,
    passed: bool,
}

struct CodeAnalyzer {
    source_code: String,
    metrics_results: Vec<MetricResult>,
}

impl CodeAnalyzer {
    fn new(source_code: &str) -> CodeAnalyzer {
        CodeAnalyzer {
            source_code: source_code.to_string(),
            metrics_results: Vec::new(),
        }
    }

    fn read_env_var(key: &str) -> Option<String> {
        match env::var(key) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn load_source_file(file_path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }

    fn lines_of_code(&self) -> usize {
        self.source_code.lines().count()
    }

    fn analyze(&mut self) {
        let loc = self.lines_of_code();
        self.metrics_results.push(MetricResult {
            name: "Lines of Code".to_string(),
            score: loc as f64,
            passed: loc > 0,
        });
    }

    fn output_results(&self) {
        for result in &self.metrics_results {
            println!("Metric: {}, Score: {}, Passed: {}", result.name, result.score, result.passed);
        }
    }
}

fn main() {
    match CodeAnalyzer::read_env_var("CODE_ANALYZER_ENV_FILE") {
        Some(env_file) => {
            if dotenv::dotenv().is_err() {
                println!("Warning: Failed to load .env file");
            }
            println!("Loaded env from {}", env_file);
        },
        None => println!("No custom environment file specified."),
    }

    let file_path_str = "path_to_your_source_code.rs"; 
    let file_path = Path::new(file_path_str);
    let source_code = match CodeAnalyzer::load_source_file(file_path) {
        Ok(code) => code,
        Err(e) => {
            println!("Failed to load source code from {}: {}", file_path.display(), e);
            process::exit(1);
        },
    };

    let mut analyzer = CodeAnalyzer::new(&source_code);

    analyzer.analyze();
    analyzer.output_results();
}