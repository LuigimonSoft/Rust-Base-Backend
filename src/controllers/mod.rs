mod base_controller;

use std::convert::Infallible;
use std::sync::Arc;

use crate::repositories::base_Repository::InMemoryBaseRepository;
use crate::services::base_service::BaseServiceImpl;
use crate::config::Config;

pub fn base_routes(config: Arc<Config>) -> impl warp::Filter<Extract = impl warp::Reply, Error = Infallible> +Clone {
  let repository = InMemoryBaseRepository::new();
  let service = BaseServiceImpl::new(repository);
  let controller = base_controller::BaseController::new(service, Arc::clone(&config));

  controller.routes()
}
