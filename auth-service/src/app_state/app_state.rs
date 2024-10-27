use crate::{BannedTokenStoreType, TwoFACodeStoreType, UserStoreType};

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
    pub token_store: BannedTokenStoreType, 
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType, 
        token_store: BannedTokenStoreType, 
        two_fa_code_store: TwoFACodeStoreType
    ) -> Self {
        Self { user_store, token_store, two_fa_code_store }
    }
}