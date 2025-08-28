use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Option<Arc<PgPool>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: None,
        }
    }

    pub fn set_db_pool(&mut self, pool: PgPool) {
        self.db = Some(Arc::new(pool));
    }

    pub fn get_db_pool(&self) -> Option<Arc<PgPool>> {
        self.db.as_ref().map(Arc::clone)
    }
}
