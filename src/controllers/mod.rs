pub mod base_controller;
pub mod auth_controller;

use warp::Rejection;
use std::sync::Arc;

use crate::repositories::base_Repository::InMemoryBaseRepository;
use crate::services::base_service::BaseServiceImpl;
use crate::config::Config;
use crate::router::Router;


pub fn base_routes(config: Arc<Config>) -> impl warp::Filter<Extract = impl warp::Reply, Error = Rejection> +Clone {
  let repository = InMemoryBaseRepository::new();
  let service = BaseServiceImpl::new(repository);
  let router = Router::new(service, Arc::clone(&config));

  router.routes()
}
