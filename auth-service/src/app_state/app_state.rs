use crate::{UserStoreType, BannedTokenStoreType};

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub token_store: BannedTokenStoreType, 
}

impl AppState {
    pub fn new(user_store: UserStoreType, token_store: BannedTokenStoreType) -> Self {
        Self { user_store, token_store }
    }
}