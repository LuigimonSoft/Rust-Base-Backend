#![allow(dead_code, unused_imports, unused_variables)]
use std::sync::Arc;
use crate::config;

#[tokio::test]
async fn test_index_html_is_served() {
    dotenv::dotenv().ok();
    let config = Arc::new(config::Config::from_env());
    let static_files = warp::fs::dir(config.static_dir.clone());

    let res = warp::test::request()
        .method("GET")
        .path("/index.html")
        .reply(&static_files)
        .await;

    assert_eq!(res.status(), 200);
    assert_eq!(res.headers().get("content-type").unwrap(), "text/html");
    assert!(!res.body().is_empty());
    assert!(String::from_utf8_lossy(res.body()).contains("Static file served"));
}
