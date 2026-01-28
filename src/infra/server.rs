use crate::config::ServerConfig;
use crate::core::AppState;
use crate::infra::latency::LatencyOnResponse;
use axum::Router;
use axum::extract::{DefaultBodyLimit, Request};
use bytesize::ByteSize;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
pub struct Server {
    config: &'static ServerConfig,
}
impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }
    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);
        let port = self.config.port();
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        tracing::info!("listening on {}", listener.local_addr()?);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }
    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        // 请求超时保护
        let timeout = TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(60),
        );
        // 限制 Body 大小
        let body_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        // CORS 配置
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_headers(cors::Any)
            .allow_methods(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));
        // 统一路径格式，去除尾部斜杠
        let normalize_path = NormalizePathLayer::trim_trailing_slash();
        // 链路追踪与响应耗时统计
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("API Request",id = %id, method = %method,path = %path ,status = tracing::field::Empty, latency = tracing::field::Empty, )
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        Router::new()
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(tracing)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}
