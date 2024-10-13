pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;

use crate::services::hashmap_user_store::HashmapUserStore;

use std::sync::Arc;

use tokio::sync::RwLock;
use axum::{serve::Serve, Router};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub fn new(server: Serve<Router, Router>, address: String) -> Self {
        Self { server, address }
    }
}

pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

