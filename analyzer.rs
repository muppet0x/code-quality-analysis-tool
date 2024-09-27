use std::env;
use std::fs;
use std::path::Path;

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
        env::var(key).ok()
    }

    fn load_source_file(file_path: &Path) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }

    fn lines_of_code(&self) -> usize {
        self.source_code.lines().count()
    }

    fn analyze(&mut self) {
        let loc = self.lines_of_code();
        let loc_metric = MetricResult {
            name: "Lines of Code".to_string(),
            score: loc as f64,
            passed: loc > 0,
        };
        self.metrics_results.push(loc_metric);
    }

    fn output_results(&self) {
        for result in &self.metrics_results {
            println!("Metric: {}, Score: {}, Passed: {}", result.name, result.score, result.passed);
        }
    }
}

fn main() {
    if let Some(env_file) = CodeAnalyzer::read_env_var("CODE_ANALYZER_ENV_FILE") {
        dotenv::dotenv().ok();
        println!("Loaded env from {}", env_file);
    }

    let source_code = "fn main() {\n    println!(\"Hello, world!\");\n}";
    let mut analyzer = CodeAnalyzer::new(source_code);

    analyzer.analyze();

    analyzer.output_results();
}