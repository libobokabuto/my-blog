use app::{content, shell};
use axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use leptos::config::get_configuration;
use leptos_axum::{handle_server_fns, render_app_to_stream};
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    _ = any_spawner::Executor::init_tokio();

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("server=info,tower_http=info")),
        )
        .init();

    let conf = get_configuration(Some("Cargo.toml")).expect("读取 Leptos 配置失败");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.to_string();
    let shell_options = leptos_options.clone();
    let site_url = format!("http://{}", leptos_options.site_addr);

    let app = Router::new()
        .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
        .route("/rss.xml", get({
            let site_url = site_url.clone();
            move || rss_handler(site_url.clone())
        }))
        .route("/sitemap.xml", get({
            let site_url = site_url.clone();
            move || sitemap_handler(site_url.clone())
        }))
        .nest_service("/pkg", ServeDir::new(format!("{site_root}/pkg")))
        .fallback(render_app_to_stream(move || shell(shell_options.clone())))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("绑定监听端口失败");

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.expect("服务启动失败");
}

async fn server_fn_handler(request: Request<Body>) -> impl IntoResponse {
    handle_server_fns(request).await
}

async fn rss_handler(site_url: String) -> impl IntoResponse {
    match content::build_rss_xml(&site_url).await {
        Ok(body) => (
            [(header::CONTENT_TYPE, HeaderValue::from_static("application/rss+xml; charset=utf-8"))],
            body,
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("生成 RSS 失败：{error}"),
        )
            .into_response(),
    }
}

async fn sitemap_handler(site_url: String) -> impl IntoResponse {
    match content::build_sitemap_xml(&site_url).await {
        Ok(body) => (
            [(header::CONTENT_TYPE, HeaderValue::from_static("application/xml; charset=utf-8"))],
            body,
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("生成 sitemap 失败：{error}"),
        )
            .into_response(),
    }
}
