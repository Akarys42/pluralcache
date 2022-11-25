mod models;
mod traits;
mod implementations;

use std::sync::Arc;
use tokio::sync::Mutex;
use implementations::in_memory_cache::InMemoryCache;
use implementations::origin_api::OriginApi;
use implementations::controller::Controller;

fn main() {
    let memory_cache: Arc<Mutex<&mut InMemoryCache>> = Arc::new(Mutex::new(Box::leak(Box::new(InMemoryCache::new()))));
    let origin_api: Arc<Mutex<&mut OriginApi>> = Arc::new(Mutex::new(Box::leak(Box::new(OriginApi::new("https://api.pluralkit.me/v2".to_string())))));
    let mut controller = Controller::new();

    controller.add_notifier(memory_cache.clone());
    controller.add_provider(memory_cache.clone());
    controller.add_provider(origin_api.clone());
}
