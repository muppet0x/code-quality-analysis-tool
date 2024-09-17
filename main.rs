async fn analyze_code_batch(codes: Vec<String>) -> impl Responder {
    let results = send_batch_for_analysis(codes).await;
    HttpResponse::Ok().json(results)
}

async fn send_batch_for_analysis(codes: Vec<String>) -> Vec<AnalysisResult> {
    vec![]
}

struct AnalysisResult {
    code: String,
    status: String,
}