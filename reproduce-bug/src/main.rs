use reqwest_tracing::reqwest_otel_span;

fn main() {
    let req: reqwest::Request = todo!();

    let span = reqwest_otel_span!(name = "backend_request", req);
}
