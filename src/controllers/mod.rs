pub mod auth_controller;
pub mod base_controller;

use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection};

use crate::config::Config;
use crate::errors::ApiError;
use crate::repositories::base_Repository::InMemoryBaseRepository;
use crate::repositories::credentials_repository::InMemoryCredentialRepository;
use crate::repositories::token_repository::InMemoryTokenRepository;
use crate::router::Router;
use crate::services::auth_service::{AuthService, AuthServiceImpl};
use crate::services::base_service::{BaseService, BaseServiceImpl};

pub fn routes(
    config: Arc<Config>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let base_repository = InMemoryBaseRepository::new();
    let base_service = BaseServiceImpl::new(base_repository);
    let base_router = Router::new(base_service, Arc::clone(&config)).routes();

    let token_repository = InMemoryTokenRepository::new();
    let credential_repository = InMemoryCredentialRepository::new();
    let auth_service = AuthServiceImpl::new(token_repository, credential_repository);
    let auth_routes = build_auth_routes(auth_service, Arc::clone(&config));

    base_router.or(auth_routes)
}

fn build_auth_routes<S: AuthService + Send + Sync + 'static>(
    service: S,
    config: Arc<Config>,
) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let service = Arc::new(service);
    let api_base = config.api_base.trim_matches('/').to_string();
    let segments: Vec<String> = api_base.split('/').map(|s| s.to_string()).collect();

    let mut api_path = warp::path(segments[0].clone()).boxed();
    for seg in &segments[1..] {
        api_path = api_path.and(warp::path(seg.clone())).boxed();
    }

    let auth_token = warp::post()
        .and(api_path.clone())
        .and(warp::path("auth"))
        .and(warp::path("token"))
        .and(warp::path::end())
        .and(with_auth_service(Arc::clone(&service)))
        .and(warp::body::json())
        .and_then(auth_controller::generate_token);

    let protected = warp::get()
        .and(api_path.clone())
        .and(warp::path("protected"))
        .and(warp::path::end())
        .and(authorize(Arc::clone(&service)))
        .and_then(auth_controller::protected_endpoint);

    auth_token.or(protected)
}

fn with_auth_service<S: AuthService + Send + Sync + 'static>(
    service: Arc<S>,
) -> impl Filter<Extract = (Arc<S>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}

fn authorize<S: AuthService + Send + Sync + 'static>(
    service: Arc<S>,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and_then(move |header: Option<String>| {
            let svc = Arc::clone(&service);
            async move {
                if let Some(h) = header {
                    if let Some(token) = h.strip_prefix("Bearer ") {
                        if svc.validate_token(token).await {
                            return Ok(());
                        }
                    }
                }
                Err(warp::reject::custom(ApiError::Unauthorized))
            }
        })
        .untuple_one()
}
