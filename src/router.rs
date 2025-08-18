use warp::Filter;
use crate::services::base_service::BaseService;
use crate::config::Config;
use std::sync::Arc;
use std::convert::Infallible;
use warp::Rejection;
use crate::controllers::base_controller::{handle_get_messages, handle_create_message, handle_search_messages};

pub struct Router<S: BaseService> {
  service: Arc<S>,
  config: Arc<Config>
}

impl<S: BaseService + Send + Sync + 'static> Router<S> {
  pub fn new(service: S, config: Arc<Config>) -> Self {
    Self {
      service: Arc::new(service),
      config
    }
  }


pub fn routes(&self) -> impl Filter<Extract = impl warp::Reply, Error = Rejection> + Clone {
    let service = self.service.clone();
    let api_base = self.config.api_base.trim_matches('/').to_string();
    let api_segments: Vec<String> = api_base.split('/').map(|s| s.to_string()).collect();
    let api_path_complete: String = api_base.clone() + ("/messages");

    let mut api_path = warp::path(api_segments[0].clone()).boxed();
    for segment in &api_segments[1..] {
        api_path = api_path.and(warp::path(segment.clone())).boxed();
    }

    let get_messages = warp::get()
        .and(api_path.clone())
        .and(warp::path("messages"))
        .and(warp::path::end())
        .and(with_service(Arc::clone(&service)))
        .and_then(handle_get_messages);
    
    let add_message = warp::post()
        .and(api_path.clone())
        .and(warp::path("messages"))
        .and(warp::path::end())
        .and(crate::validators::base_validator::validate_create_message(Some(api_path_complete.clone())))
        .and(with_service(Arc::clone(&service)))
        .and_then(handle_create_message);

    let search_messages = warp::get()
            .and(api_path.clone())
            .and(warp::path("messages"))
            .and(warp::path::param::<String>())
            .and(with_service(Arc::clone(&service)))
            .and_then(handle_search_messages);

        get_messages
            .or(add_message)
            .or(search_messages)
  }
}

fn with_service<S: BaseService + Send + Sync + 'static>(
    service: Arc<S>,
) -> impl Filter<Extract = (Arc<S>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&service))
}