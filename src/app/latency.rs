use std::{fmt::Display, time::Duration};

use tower_http::trace::OnResponse;

#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(
        self,
        response: &axum::http::Response<B>,
        latency: std::time::Duration,
        _span: &tracing::Span,
    ) {
        tracing::info!(
            latency=%Latency(latency),
            status=response.status().as_u16(),
            "finished request"
        );
    }
}
struct Latency(Duration);
impl Display for Latency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            return write!(f, "{}ms", self.0.as_millis());
        } else {
            write!(f, "{}μs", self.0.as_micros())
        }
    }
}
