#![allow(dead_code, unused_imports, unused_variables)]
use warp::Reply;

use crate::controllers::protected_controller::protected_endpoint;

#[tokio::test]
async fn handler_protected_endpoint() {
    let reply = protected_endpoint().await.unwrap().into_response();
    assert_eq!(reply.status(), 200);
}
