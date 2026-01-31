use mongodb::Database;
use std::sync::Arc;

pub struct InnerState {
    pub db: Database,
}

pub type AppState = Arc<InnerState>;

impl InnerState {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}
